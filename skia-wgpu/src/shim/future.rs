//! WebGPU futures: the callbacks of asynchronous operations and when they may fire.
//!
//! A WebGPU operation such as `wgpuBufferMapAsync` returns a future and takes a callback with a
//! callback mode. The mode decides where the callback fires once the operation completes:
//! `WaitAnyOnly` only inside `wgpuInstanceWaitAny` on that future, `AllowProcessEvents` also
//! inside `wgpuInstanceProcessEvents`, and `AllowSpontaneous` as soon as `wgpu` reports the
//! completion. `wgpu` reports completions from inside `wgpu::Device::poll`, so the instance polls
//! its devices while it waits.

use std::collections::HashMap;
use std::ffi::c_void;
use std::fmt;
use std::pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Weak};
use std::task;

use skia_bindings as sb;

/// The futures of one instance, by id.
///
/// Only the instance holds them. The device, the queue, the buffers, and the shader modules hold
/// the instance, so the callbacks that wait in the futures are dropped, and therefore cancelled,
/// when the last of these objects is released. A waiting callback that owns a handle to one of
/// these objects keeps the instance alive, so it is never cancelled. Skia's waiting callbacks own
/// no such handle.
#[derive(Debug, Default)]
pub struct Futures {
    next_id: AtomicU64,
    entries: Mutex<HashMap<u64, Entry>>,
}

/// A future whose callback has not fired yet. A future without an entry has fired its callback, or
/// was never registered.
enum Entry {
    Pending,
    /// The operation completed, and the callback waits for `WaitAny` or `ProcessEvents`.
    Ready {
        mode: sb::WGPUCallbackMode,
        fire: Box<dyn FnOnce() + Send>,
    },
}

/// Shows the state and the mode. A callback has no `Debug` representation.
impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Entry::Pending => f.write_str("Pending"),
            Entry::Ready { mode, .. } => f
                .debug_struct("Ready")
                .field("mode", mode)
                .finish_non_exhaustive(),
        }
    }
}

impl Futures {
    /// Registers a new future and returns it with the [`Completion`] that completes it.
    ///
    /// `call` calls the C callback with the operation's result, or with the status
    /// `CallbackCancelled` for `None`. It is called exactly once, see [`Callback`].
    pub fn register<R>(
        self: &Arc<Self>,
        mode: sb::WGPUCallbackMode,
        call: impl FnOnce(Option<R>) + Send + 'static,
    ) -> (sb::WGPUFuture, Completion<R>) {
        // Id 0 is WebGPU's null future.
        let id = self.next_id.fetch_add(1, Ordering::Relaxed) + 1;
        self.entries.lock().unwrap().insert(id, Entry::Pending);
        let completion = Completion {
            futures: Arc::downgrade(self),
            id,
            mode,
            callback: Callback {
                call: Some(Box::new(call)),
            },
        };
        (sb::WGPUFuture { id }, completion)
    }

    /// Fires the callbacks of the completed futures in `infos`, marks them `completed`, and
    /// returns whether any of them completed.
    pub fn fire_waited(&self, infos: &mut [sb::WGPUFutureWaitInfo]) -> bool {
        let mut any_completed = false;
        for info in infos {
            let ready = {
                let mut entries = self.entries.lock().unwrap();
                match entries.get(&info.future.id) {
                    Some(Entry::Pending) => None,
                    Some(Entry::Ready { .. }) => entries.remove(&info.future.id),
                    None => {
                        info.completed = 1;
                        any_completed = true;
                        continue;
                    }
                }
            };
            if let Some(Entry::Ready { fire, .. }) = ready {
                // Fired outside the lock, because the callback may call back into the shim.
                fire();
                info.completed = 1;
                any_completed = true;
            }
        }
        any_completed
    }

    /// Fires the callbacks of all completed futures whose mode allows `ProcessEvents`.
    pub fn fire_process_events(&self) {
        let ready: Vec<_> = {
            let mut entries = self.entries.lock().unwrap();
            let ids: Vec<u64> = entries
                .iter()
                .filter(|(_, entry)| {
                    matches!(entry, Entry::Ready { mode, .. }
                        if *mode == sb::WGPUCallbackMode::WGPUCallbackMode_AllowProcessEvents)
                })
                .map(|(id, _)| *id)
                .collect();
            ids.into_iter()
                .filter_map(|id| entries.remove(&id))
                .collect()
        };
        for entry in ready {
            if let Entry::Ready { fire, .. } = entry {
                fire();
            }
        }
    }
}

/// Completes one registered future with a result of type `R`. It refers to the futures weakly, so
/// that the operations that still run do not keep the futures of a released instance alive.
#[derive(Debug)]
pub struct Completion<R> {
    futures: Weak<Futures>,
    id: u64,
    mode: sb::WGPUCallbackMode,
    callback: Callback<R>,
}

impl<R: Send + 'static> Completion<R> {
    /// Completes the future. The callback fires with `result` now when the mode allows
    /// spontaneous callbacks, and otherwise when the future is waited on or events are processed.
    pub fn complete(self, result: R) {
        let Completion {
            futures,
            id,
            mode,
            callback,
        } = self;
        // When the instance has been released, the callback is dropped here, and cancelled.
        let Some(futures) = futures.upgrade() else {
            return;
        };
        let fire = move || callback.fire(result);
        if mode == sb::WGPUCallbackMode::WGPUCallbackMode_AllowSpontaneous {
            futures.entries.lock().unwrap().remove(&id);
            fire();
        } else {
            let fire = Box::new(fire);
            let entry = Entry::Ready { mode, fire };
            futures.entries.lock().unwrap().insert(id, entry);
        }
    }
}

/// A C callback, which WebGPU calls exactly once.
///
/// [`Callback::fire`] calls it with the operation's result, and dropping it unfired calls it with
/// the status `CallbackCancelled`. A callback is dropped unfired when its instance is released
/// before the operation completes, or before the callback's mode lets it fire. `wgpu` calls the
/// closure of every operation it accepts, so a [`Completion`] is not dropped unused.
struct Callback<R> {
    call: Option<Box<dyn FnOnce(Option<R>) + Send>>,
}

impl<R> Callback<R> {
    fn fire(mut self, result: R) {
        let call = self.call.take().expect("a callback fires once");
        call(Some(result));
    }
}

impl<R> Drop for Callback<R> {
    fn drop(&mut self) {
        if let Some(call) = self.call.take() {
            call(None);
        }
    }
}

/// A callback has no `Debug` representation.
impl<R> fmt::Debug for Callback<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Callback").finish_non_exhaustive()
    }
}

/// The two userdata pointers of a C callback, which Skia owns and passes back unchanged.
#[derive(Debug, Clone, Copy)]
pub struct Userdata(pub *mut c_void, pub *mut c_void);

// Skia passes the userdata pointers back to its own callback, on whatever thread the callback
// fires. Dawn does the same, so Skia's callbacks already expect it.
unsafe impl Send for Userdata {}

/// Resolves a `wgpu` future that is ready at once. `wgpu`'s native backends resolve the futures
/// that the shim waits on (adapters, devices, error scopes, and compilation info) when they are
/// created, so nothing would ever wake a pending one.
pub fn block_on<F: Future>(future: F) -> F::Output {
    let mut context = task::Context::from_waker(task::Waker::noop());
    match pin::pin!(future).poll(&mut context) {
        task::Poll::Ready(output) => output,
        task::Poll::Pending => panic!("a wgpu future that the shim waits on is not ready"),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use skia_bindings as sb;

    use super::{Completion, Futures};

    /// The results that a callback received, `None` for a cancellation.
    type Log = Arc<Mutex<Vec<Option<&'static str>>>>;

    /// A callback that waits for `WaitAny` when its instance is released fires once, with
    /// `CallbackCancelled`, although its operation completed.
    #[test]
    fn a_waiting_callback_is_cancelled_with_its_instance() {
        let futures = Arc::new(Futures::default());
        let (log, completion) =
            register(&futures, sb::WGPUCallbackMode::WGPUCallbackMode_WaitAnyOnly);
        completion.complete("completed");
        drop(futures);
        assert_eq!(*log.lock().unwrap(), [None]);
    }

    /// A callback whose instance was released while its operation ran fires once, with
    /// `CallbackCancelled`, when the operation completes later.
    #[test]
    fn a_result_after_the_release_cancels_the_callback() {
        let futures = Arc::new(Futures::default());
        let (log, completion) = register(
            &futures,
            sb::WGPUCallbackMode::WGPUCallbackMode_AllowSpontaneous,
        );
        drop(futures);
        completion.complete("completed");
        assert_eq!(*log.lock().unwrap(), [None]);
    }

    /// Registers a future whose callback logs each result it receives.
    fn register(
        futures: &Arc<Futures>,
        mode: sb::WGPUCallbackMode,
    ) -> (Log, Completion<&'static str>) {
        let log = Log::default();
        let logged = log.clone();
        let (_, completion) =
            futures.register(mode, move |result| logged.lock().unwrap().push(result));
        (log, completion)
    }
}

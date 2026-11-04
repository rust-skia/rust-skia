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
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::task;

use skia_bindings as sb;

/// The futures of one instance, by id.
#[derive(Debug, Default)]
pub struct Futures {
    next_id: AtomicU64,
    entries: Mutex<HashMap<u64, Entry>>,
}

/// A future whose callback has not fired yet. A future without an entry has completed.
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
    pub fn register(self: &Arc<Self>, mode: sb::WGPUCallbackMode) -> (sb::WGPUFuture, Completion) {
        // Id 0 is WebGPU's null future.
        let id = self.next_id.fetch_add(1, Ordering::Relaxed) + 1;
        self.entries.lock().unwrap().insert(id, Entry::Pending);
        let completion = Completion {
            futures: self.clone(),
            id,
            mode,
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

/// Completes one registered future.
#[derive(Debug)]
pub struct Completion {
    futures: Arc<Futures>,
    id: u64,
    mode: sb::WGPUCallbackMode,
}

impl Completion {
    /// Completes the future. `fire` invokes the C callback with the operation's result. It runs
    /// now when the mode allows spontaneous callbacks, and otherwise when the future is waited on
    /// or events are processed.
    pub fn complete(self, fire: impl FnOnce() + Send + 'static) {
        if self.mode == sb::WGPUCallbackMode::WGPUCallbackMode_AllowSpontaneous {
            self.futures.entries.lock().unwrap().remove(&self.id);
            fire();
        } else {
            self.futures.entries.lock().unwrap().insert(
                self.id,
                Entry::Ready {
                    mode: self.mode,
                    fire: Box::new(fire),
                },
            );
        }
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

use crate::prelude::*;
use skia_bindings as sb;

// Re-export backend types from skia_bindings
pub use sb::skgpu_BackendApi as BackendApi;
pub use sb::skgpu_Budgeted as Budgeted;
pub use sb::skgpu_Mipmapped as Mipmapped;

/// Status of recording insertion
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum InsertStatus {
    /// Recording was successfully inserted
    Success = 0,
    /// Recording failed to insert
    Failure = 1,
}

impl From<i32> for InsertStatus {
    fn from(value: i32) -> Self {
        match value {
            0 => InsertStatus::Success,
            _ => InsertStatus::Failure,
        }
    }
}

/// Configuration for recorder creation
#[derive(Debug)]
pub struct RecorderOptions {
    inner: sb::skgpu_graphite_RecorderOptions,
}

impl Default for RecorderOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl RecorderOptions {
    /// Create new recorder options with default settings
    pub fn new() -> Self {
        let mut inner = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        // Initialize with default values - the C++ constructor will set appropriate defaults
        unsafe {
            std::ptr::write(&mut inner, std::mem::zeroed());
        }
        Self { inner }
    }

    pub(crate) fn native(&self) -> &sb::skgpu_graphite_RecorderOptions {
        &self.inner
    }

    #[allow(dead_code)]
    pub(crate) fn native_mut(&mut self) -> &mut sb::skgpu_graphite_RecorderOptions {
        &mut self.inner
    }
}

native_transmutable!(sb::skgpu_graphite_RecorderOptions, RecorderOptions);

/// Information for inserting a recording into the context
#[derive(Debug)]
pub struct InsertRecordingInfo {
    inner: sb::skgpu_graphite_InsertRecordingInfo,
}

impl InsertRecordingInfo {
    /// Create insert recording info for a recording
    pub fn new(recording: &crate::graphite::Recording) -> Self {
        let mut inner: sb::skgpu_graphite_InsertRecordingInfo =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        unsafe {
            std::ptr::write(&mut inner, std::mem::zeroed());
        }
        inner.fRecording = recording.native() as *const _ as *mut _;
        Self { inner }
    }

    pub(crate) fn native(&self) -> &sb::skgpu_graphite_InsertRecordingInfo {
        &self.inner
    }
}

/// Information for submitting work to the GPU
#[derive(Debug)]
pub struct SubmitInfo {
    inner: sb::skgpu_graphite_SubmitInfo,
}

impl Default for SubmitInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl SubmitInfo {
    /// Create new submit info with default settings
    pub fn new() -> Self {
        let inner = unsafe {
            let mut inner = std::mem::MaybeUninit::uninit();
            std::ptr::write(&mut inner, std::mem::zeroed());
            inner.assume_init()
        };
        Self { inner }
    }

    pub(crate) fn native(&self) -> &sb::skgpu_graphite_SubmitInfo {
        &self.inner
    }
}

/// Synchronization mode for GPU operations
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SyncToCpu {
    /// Don't wait for GPU completion
    No = 0,
    /// Wait for GPU operations to complete
    Yes = 1,
}

impl From<bool> for SyncToCpu {
    fn from(sync: bool) -> Self {
        if sync {
            SyncToCpu::Yes
        } else {
            SyncToCpu::No
        }
    }
}

impl From<SyncToCpu> for bool {
    fn from(sync: SyncToCpu) -> bool {
        match sync {
            SyncToCpu::Yes => true,
            SyncToCpu::No => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_status_conversion() {
        assert_eq!(InsertStatus::from(0), InsertStatus::Success);
        assert_eq!(InsertStatus::from(1), InsertStatus::Failure);
        assert_eq!(InsertStatus::from(999), InsertStatus::Failure);
    }

    #[test]
    fn test_sync_to_cpu_conversion() {
        assert_eq!(SyncToCpu::from(true), SyncToCpu::Yes);
        assert_eq!(SyncToCpu::from(false), SyncToCpu::No);
        assert_eq!(bool::from(SyncToCpu::Yes), true);
        assert_eq!(bool::from(SyncToCpu::No), false);
    }

    #[test]
    fn test_recorder_options_creation() {
        let options = RecorderOptions::new();
        let _default_options = RecorderOptions::default();
        // Should not panic and should create valid options
        let _ = format!("{:?}", options);
    }

    #[test]
    fn test_submit_info_creation() {
        let info = SubmitInfo::new();
        let _default_info = SubmitInfo::default();
        // Should not panic and should create valid info
        let _ = format!("{:?}", info);
    }
}

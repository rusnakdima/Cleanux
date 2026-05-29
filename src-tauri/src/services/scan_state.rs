use parking_lot::Mutex;
use std::sync::Arc;

pub struct ScanState {
    pub is_cancelled: Arc<Mutex<bool>>,
}

unsafe impl Send for ScanState {}
unsafe impl Sync for ScanState {}

impl ScanState {
    pub fn new() -> Self {
        Self {
            is_cancelled: Arc::new(Mutex::new(false)),
        }
    }

    pub fn cancel(&self) {
        *self.is_cancelled.lock() = true;
    }

    pub fn is_cancelled(&self) -> bool {
        *self.is_cancelled.lock()
    }

    pub fn reset(&self) {
        *self.is_cancelled.lock() = false;
    }
}

impl Default for ScanState {
    fn default() -> Self {
        Self::new()
    }
}

static SCAN_STATE: std::sync::OnceLock<ScanState> = std::sync::OnceLock::new();

pub fn get_scan_state() -> &'static ScanState {
    SCAN_STATE.get_or_init(|| ScanState::new())
}
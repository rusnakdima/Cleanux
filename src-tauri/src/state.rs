use std::sync::Mutex;

pub struct AppState {
    pub automation_service: Mutex<()>,
    pub backup_service: Mutex<()>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            automation_service: Mutex::new(()),
            backup_service: Mutex::new(()),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
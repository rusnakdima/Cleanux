//! System information utilities

use sysinfo::{Disks, System};

pub struct SysInfo {
    system: System,
}

impl SysInfo {
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
        }
    }

    pub fn refresh(&mut self) {
        self.system.refresh_all();
    }

    pub fn cpu_usage(&self) -> f32 {
        self.system.global_cpu_usage()
    }

    pub fn memory_used(&self) -> u64 {
        self.system.used_memory()
    }

    pub fn memory_total(&self) -> u64 {
        self.system.total_memory()
    }

    pub fn disk_usage(&self) -> Vec<(String, u64, u64)> {
        let disks = Disks::new_with_refreshed_list();
        disks
            .iter()
            .map(|d| {
                let total = d.total_space();
                let available = d.available_space();
                let used = total.saturating_sub(available);
                (d.mount_point().to_string_lossy().to_string(), used, total)
            })
            .collect()
    }
}

impl Default for SysInfo {
    fn default() -> Self {
        Self::new()
    }
}

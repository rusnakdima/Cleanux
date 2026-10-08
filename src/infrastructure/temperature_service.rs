//! Temperature monitoring service for Cleanux
//!
//! Provides CPU and GPU temperature readings using Linux sysfs and vendor tools.

use crate::infrastructure::sys_utils;
use tokio::runtime::Runtime;

/// Service for reading CPU and GPU temperatures.
pub struct TemperatureService {
  runtime: Runtime,
}

impl TemperatureService {
  pub fn new() -> Self {
    let runtime = Runtime::new().expect("failed to create tokio runtime");
    Self { runtime }
  }

  /// Get current CPU temperature in Celsius.
  ///
  /// Reads from `/sys/class/thermal/thermal_zone*/temp`.
  pub fn get_cpu_temp(&self) -> f64 {
    self
      .runtime
      .block_on(sys_utils::get_cpu_temp())
      .unwrap_or(f64::NAN)
  }

  /// Get current GPU temperature in Celsius.
  ///
  /// Probes nvidia-smi first, then falls back to amdgpu and intel sysfs.
  /// Returns `None` if no GPU is detected or temperature is unavailable.
  pub fn get_gpu_temp(&self) -> Option<f64> {
    self.runtime.block_on(sys_utils::get_gpu_temp()).ok()
  }

  /// Get both CPU and GPU temperatures in a single call.
  pub fn get_temps(&self) -> (f64, Option<f64>) {
    let cpu = self.get_cpu_temp();
    let gpu = self.get_gpu_temp();
    (cpu, gpu)
  }
}

impl Default for TemperatureService {
  fn default() -> Self {
    Self::new()
  }
}

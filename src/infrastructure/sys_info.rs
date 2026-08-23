//! System information utilities

pub struct SysInfo;

impl SysInfo {
  pub fn disk_usage(path: &str) -> Result<(u64, u64, u64), String> {
    // Returns (total, used, available) in bytes
    // This is a placeholder - actual implementation uses sysinfo crate
    Ok((0, 0, 0))
  }

  pub fn memory_usage() -> Result<(u64, u64, u64), String> {
    // Returns (total, used, available) in bytes
    Ok((0, 0, 0))
  }

  pub fn cpu_temperature() -> Result<f64, String> {
    Ok(0.0)
  }
}

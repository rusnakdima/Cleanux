//! System monitor service for Cleanux
//!
//! Provides ad-hoc network and disk I/O statistics by reading Linux procfs.

use serde::{Deserialize, Serialize};

/// Network I/O statistics — bytes sent/received and active connections.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
  /// Total bytes transmitted across all interfaces.
  pub bytes_sent: u64,
  /// Total bytes received across all interfaces.
  pub bytes_recv: u64,
  /// Number of active TCP connections.
  pub tcp_connections: usize,
  /// Number of active UDP connections.
  pub udp_connections: usize,
}

/// Disk I/O statistics — read/write bytes per second.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskIO {
  /// Read bytes per second.
  pub read_bytes_per_sec: u64,
  /// Write bytes per second.
  pub write_bytes_per_sec: u64,
}

/// Read network statistics from `/proc/net/dev` and `/proc/net/tcp`.
pub fn get_network_stats() -> NetworkStats {
  let mut bytes_sent = 0u64;
  let mut bytes_recv = 0u64;

  // Parse /proc/net/dev — skip first two header lines
  if let Ok(content) = std::fs::read_to_string("/proc/net/dev") {
    for line in content.lines().skip(2) {
      let parts: Vec<&str> = line.split_whitespace().collect();
      if parts.len() >= 10 {
        // Format: iface: rx_bytes rx_packets ... tx_bytes tx_packets
        let iface = parts[0].trim_end_matches(':');
        if iface == "lo" {
          continue;
        }
        if let (Ok(rx), Ok(tx)) = (parts[1].parse::<u64>(), parts[9].parse::<u64>()) {
          bytes_recv += rx;
          bytes_sent += tx;
        }
      }
    }
  }

  let tcp_connections = count_connections("/proc/net/tcp");
  let udp_connections = count_connections("/proc/net/udp");

  NetworkStats {
    bytes_sent,
    bytes_recv,
    tcp_connections,
    udp_connections,
  }
}

fn count_connections(path: &str) -> usize {
  std::fs::read_to_string(path)
    .map(|content| content.lines().skip(1).count())
    .unwrap_or(0)
}

/// Read disk I/O statistics from `/proc/diskstats`.
/// Returns cumulative read/write bytes since boot. For per-second rates,
/// call this twice and compute the difference divided by elapsed seconds.
pub fn get_disk_io() -> DiskIO {
  let mut read_bytes: u64 = 0;
  let mut write_bytes: u64 = 0;

  // /proc/diskstats fields (Linux kernel doc):
  //  devname rd_ios rd_merges rd_sectors rd_ticks wr_ios wr_merges wr_sectors wr_ticks
  if let Ok(content) = std::fs::read_to_string("/proc/diskstats") {
    for line in content.lines() {
      let parts: Vec<&str> = line.split_whitespace().collect();
      if parts.len() >= 9 {
        // Skip loop/ram devices
        let dev = parts[2];
        if dev.starts_with("loop") || dev.starts_with("ram") {
          continue;
        }
        // Sector size is 512 bytes
        if let Ok(sectors_read) = parts[3].parse::<u64>() {
          read_bytes += sectors_read * 512;
        }
        if let Ok(sectors_written) = parts[7].parse::<u64>() {
          write_bytes += sectors_written * 512;
        }
      }
    }
  }

  DiskIO {
    read_bytes_per_sec: read_bytes,
    write_bytes_per_sec: write_bytes,
  }
}

/* sys lib */
use std::process::{Command, Output};

pub fn stderr_message(output: &Output) -> String {
  String::from_utf8_lossy(&output.stderr).trim().to_string()
}

pub fn pkexec_rm_paths(paths: &[String]) -> Result<Output, std::io::Error> {
  let mut cmd = Command::new("pkexec");
  cmd.arg("rm").arg("-f");
  for path in paths {
    cmd.arg(path);
  }
  cmd.output()
}

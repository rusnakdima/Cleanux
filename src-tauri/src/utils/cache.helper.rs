use std::sync::Mutex;
use std::time::{Duration, Instant};
pub struct TimedCache<T: Clone> {
  data: Mutex<Option<(T, Instant)>>,
  ttl: Duration,
}
impl<T: Clone> Clone for TimedCache<T> {
  fn clone(&self) -> Self {
    TimedCache {
      data: Mutex::new(None),
      ttl: self.ttl,
    }
  }
}
impl<T: Clone> TimedCache<T> {
  pub fn new(ttl: Duration) -> Self {
    Self {
      data: Mutex::new(None),
      ttl,
    }
  }
  pub fn get(&self) -> Option<T> {
    let guard = self.data.lock().unwrap();
    match guard.as_ref() {
      Some((value, instant)) if instant.elapsed() < self.ttl => Some(value.clone()),
      _ => None,
    }
  }
  pub fn set(&self, value: T) {
    let mut guard = self.data.lock().unwrap();
    *guard = Some((value, Instant::now()));
  }
  pub fn clear(&self) {
    let mut guard = self.data.lock().unwrap();
    *guard = None;
  }
}

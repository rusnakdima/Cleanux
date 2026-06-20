/* sys lib */
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct SystemServiceModel {
  pub name: String,
  pub description: String,
  pub status: String,
  pub is_running: bool,
}

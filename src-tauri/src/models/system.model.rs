/* sys lib */
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct SystemServiceModel {
  pub name: String,
  pub description: String,
  pub status: String,
  pub isRunning: bool,
}

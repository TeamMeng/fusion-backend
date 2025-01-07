mod user;

use serde::{Deserialize, Serialize};

pub use user::{signin_handler, signup_handler};

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    pub success: bool,
    pub message: String,
}

impl Output {
    pub fn new(success: bool, message: impl Into<String>) -> Self {
        Self {
            success,
            message: message.into(),
        }
    }
}

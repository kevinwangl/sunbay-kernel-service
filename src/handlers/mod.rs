pub mod emv;
pub mod health;
pub mod transaction;

use std::sync::Arc;

use crate::services::{BackendClient, EmvProcessor};

pub use emv::*;
pub use health::*;
pub use transaction::*;

/// Application State
#[derive(Clone)]
pub struct AppState {
    pub emv_processor: Arc<EmvProcessor>,
    pub backend_client: Arc<BackendClient>,
}

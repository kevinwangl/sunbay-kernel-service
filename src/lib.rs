// Library entry point for sunbay-kernel-service
// This module exposes EMV processing functionality for both WASM and native use

pub mod models;
pub mod services;
pub mod utils;

#[cfg(feature = "server")]
pub mod config;
#[cfg(feature = "server")]
pub mod handlers;

// Re-export commonly used types
pub use models::emv::{ApduCommand, ApduResponse, CardData, Tlv};
pub use services::emv_processor::EmvProcessor;

#[cfg(feature = "server")]
pub use services::backend_client::BackendClient;

// WASM bindings
#[cfg(target_arch = "wasm32")]
pub mod wasm;

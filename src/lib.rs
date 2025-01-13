pub mod client;
pub mod gio;

// Main API

pub use client::Client;

// Global defaults

pub const DEFAULT_PORT: u16 = 1965;

// Debug

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
pub const VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
pub const VERSION_PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");

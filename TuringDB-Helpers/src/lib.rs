#![deny(unsafe_code)]
#![deny(missing_docs)]

//! TuringDB-Helpers is a crate that make it easier to interact with the `TuringDB` server
//! and is recommended for application developers
//!
//! #### #Usage
//!
//! `Cargo.toml` file
//!
//! ```toml
//! [dependencies]
//! bincode = #add latest version here
//! turingdb-helpers = #add latest version here
//! ```

//!
//!
mod repo;
/// Handles repo queries
pub use repo::*;
mod db;
/// Handles database queries
pub use db::*;
mod document;
/// Handles document queries
pub use document::*;
mod field;
/// Handles field queries
pub use field::*;
mod commands;
/// Handles commands queries
pub use commands::*;
mod traits;
/// Handles traits queries
pub use traits::*;

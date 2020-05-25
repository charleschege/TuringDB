#![deny(unsafe_code)]

mod engine;
mod global;
mod loggers;

//pub use engine::{REPO, READER, WRITER};
pub use global::*;
//pub use loggers::ErrorLogger;

use tai64::TAI64N;
use std::net::Shutdown;
use custom_codes::DbOps;
use anyhow::Result;

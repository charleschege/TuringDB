use serde::{Serialize, Deserialize};

mod repo;
use repo::*;

mod db;
pub use db::*;
mod document;
pub use document::*;
mod field;
pub use field::*;

const BUFFER_CAPACITY: usize = 64 * 1024; //16Kb
const BUFFER_DATA_CAPACITY: usize = 1024 * 1024 * 16; // Db cannot hold data more than 16MB in size

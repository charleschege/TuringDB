use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq)]
pub struct DocumentMethods {
    pub db: String,
    pub document: String,
    pub field: String,
    pub data: Vec<u8>,
}
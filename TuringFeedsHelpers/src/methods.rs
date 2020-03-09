use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq)]
pub struct FieldWithData {
    pub db: String,
    pub document: String,
    pub field: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq)]
pub struct FieldNoData {
    pub db: String,
    pub document: String,
    pub field: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq)]
pub struct DocumentOnly {
    pub db: String,
    pub document: String,
}
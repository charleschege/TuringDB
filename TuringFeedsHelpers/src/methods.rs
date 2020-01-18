use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq)]
pub struct DocumentMethods {
    db: String,
    document: String,
    data: Vec<u8>,
}

impl DocumentMethods {
    pub async fn new() -> Self {
        Self {
            db: String::default(),
            document: String::default(),
            data: Vec::default(),
        }
    }
    pub async fn add_db(mut self, value: &str) -> Self {
        self.db = value.to_owned();

        self
    }
    pub async fn add_document(mut self, value: &str) -> Self {
        self.document = value.to_owned();

        self
    }
    pub async fn add_data(mut self, value: Vec<u8>) -> Self {
        self.data = value;

        self
    }
    pub async fn build(self) -> Self {

        self
    }
    pub async fn get_db(&self) -> String {
        self.db.to_owned()
    }
    pub async fn get_document(&self) -> String {
        self.document.to_owned()
    }
    pub async fn get_data(&self) -> Vec<u8> {
        self.data.to_owned()
    }
}

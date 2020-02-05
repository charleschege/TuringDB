use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq)]
pub struct DocumentMethods {
    db: String,
    document: String,
    field: String,
    data: Option<Vec<u8>>,
}

impl DocumentMethods {
    pub async fn new() -> Self {
        Self {
            db: String::default(),
            document: String::default(),
            field: String::default(),
            data: Option::default(),
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
    pub async fn add_field(mut self, value: &str) -> Self {
        self.field = value.to_owned();

        self
    }
    pub async fn add_data(mut self, value: Vec<u8>) -> Self {
        self.data = Some(value);

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
        match &self.data {
            Some(val) => val.to_vec(),
            None => Vec::default(),
        }
    }
}

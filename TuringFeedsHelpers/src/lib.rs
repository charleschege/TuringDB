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
    pub async fn add_db(&mut self, value: String) -> &Self {
        self.db = value;

        self
    }
    pub async fn add_document(&mut self, value: String) -> &Self {
        self.document = value;

        self
    }
    pub async fn add_data(&mut self, value: Vec<u8>) -> &Self {
        self.data = value;

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

/// Commands to perform on the repo and its contents
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub enum TuringCommand {
    /// Initialize the Repository
    InitRepo,
    /// Delete the Repository
    DropRepo,
    /// Perform a checksum of the database
    ChecksumDatabase,
    /// Perform a checksum of the database
    ChecksumTable(String),
    /// Create a database
    CreateDatabase(String),
    /// Read contents of a database
    FetchDatabase(String),
    /// Modify a database
    ModifyDatabase(String),
    /// Delete a database
    DropDatabase(String),
    /// Create a document
    CreateDocument(DocumentMethods),
    /// Read a particular document
    FetchDocument(DocumentMethods),
    /// Updata a document
    ModifyDocument(DocumentMethods),
    /// Remove a document
    DeleteDocument(DocumentMethods),
    /// Give a default option
    Unspecified,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TFDocumentData<T>
where
    T: std::fmt::Debug + std::cmp::PartialEq,
{
    data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseMethods {
    command: TuringCommand,
    db: String,
}

impl DatabaseMethods {
    pub async fn new() -> Self {
        Self {
            command: TuringCommand::Unspecified,
            db: String::default(),
        }
    }
    pub async fn command(&mut self, value: TuringCommand) -> &Self {
        self.command = value;

        self
    }
    pub async fn db(&mut self, value: String) -> &Self {
        self.db = value;

        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentMethods {
    command: TuringCommand,
    db: String,
    document: String,
    data: Vec<u8>,
}

impl DocumentMethods {
    pub async fn new() -> Self {
        Self {
            command: TuringCommand::Unspecified,
            db: String::default(),
            document: String::default(),
            data: Vec::default(),
        }
    }
    pub async fn command(&mut self, value: TuringCommand) -> &Self {
        self.command = value;

        self
    }
    pub async fn db(&mut self, value: String) -> &Self {
        self.db = value;

        self
    }
    pub async fn document(&mut self, value: String) -> &Self {
        self.document = value;

        self
    }
    pub async fn data(&mut self, value: Vec<u8>) -> &Self {
        self.data = value;

        self
    }
}

/// Commands to perform on the repo and its contents
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TuringCommand {
    /// Initialize the Repository
    InitRepo,
    /// Delete the Repository
    DropRepo,
    /// Perform a checksum of the database
    ChecksumDatabase,
    /// Perform a checksum of the database
    ChecksumTable,
    /// Create a database
    CreateDatabase,
    /// Read contents of a database
    FetchDatabase,
    /// Modify a database
    ModifyDatabase,
    /// Delete a database
    DropDatabase,
    /// Create a document
    CreateDocument,
    /// Read a particular document
    FetchDocument,
    /// Updata a document
    ModifyDocument,
    /// Remove a document
    DeleteDocument,
    /// Give a default option
    Unspecified,
}

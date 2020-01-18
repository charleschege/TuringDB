use serde::{Deserialize, Serialize};
use crate::DocumentMethods;

/// Commands to perform on the repo and its contents by the repo owner known as `SuperUser`
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub enum SuperUserTuringCommands {
    /// Initialize the Repository
    InitRepo,
    /// Delete the Repository
    DropRepo,
    /// Perform a checksum of the database
    ChecksumDatabase(String),
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

/// Commands to perform on the repo and its contents by a privileged user
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub enum PrivilegedTuringCommands {
    /// Perform a checksum of the database
    ChecksumDatabase(String),
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

/// Commands to perform on the repo and its contents by an unprivileged user
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub enum UnprivilegedTuringCommands {    
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub enum RepoCommands {
    SuperUserTuringCommands,
    PrivilegedTuringCommands,
    UnprivilegedTuringCommands,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub enum OpsOutcome {
    Success(Option<Vec<u8>>),
    Failure(Vec<u8>),
    Stream(Vec<u8>),
}
use serde::{Deserialize, Serialize};
use crate::{DocumentMethods, OperationErrors};

/// Commands to perform on the repo and its contents by the repo owner known as `SuperUser`
#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
    FetchDatabases(String),
    /// Delete a database
    DropDatabase(String),
    /// Create a document
    CreateDocument(DocumentMethods),
    ///Insert a field into a document
    InsertField(DocumentMethods),
    /// Read a particular document
    FetchDocument(FieldLite),
    /// Remove a particular document
    RemoveDocument(FieldLite),
    /// Updata a document
    ModifyDocument(DocumentMethods),
    /// Remove a document
    DeleteDocument(DocumentMethods),
    /// Give a default option
    Unspecified,
}

/// Commands to perform on the repo and its contents by a privileged user
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct FieldLite {
    pub db: String,
    pub document: String,
    pub field: String,
}

/// Commands to perform on the repo and its contents by a privileged user
#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum RepoCommands {
    SuperUser(SuperUserTuringCommands),
    Privileged(PrivilegedTuringCommands),
    UnPrivileged(UnprivilegedTuringCommands),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum OpsOutcome {
    Success(DbOpsOutcome),
    Failure(OperationErrors),
    Stream(Vec<u8>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum DbOpsOutcome {
    Success,
    OpFailure(OperationErrors),
    SuccessWithData(Vec<u8>),
    SuccessStreamData(Vec<u8>),
}

type Key = String;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Permissions {
    SuperUser(Key),
    PrivilegedUser(Key),
    UnprivilegedUser(Key),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TuringTerminator;
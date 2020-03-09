use serde::{Deserialize, Serialize};
use crate::{DocumentOnly, FieldWithData, FieldNoData};

/// Commands to perform on the repo and its contents by the repo owner known as `SuperUser`
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TuringCommands {
    /// Initialize the Repository
    CreateRepo,
    /// Read databases in a repository
    RepoRead,
    /// Delete the Repository
    DropRepo,
    /// Create a database
    DbCreate(String),
    /// Read documents in a database
    DbRead(String),
    /// List contents of a database
    DbList(String),
    /// Delete a database
    DropDatabase(String),
    /// Create a document
    DocumentCreate(DocumentOnly),
    /// List all fields in a document
    DocumentList(DocumentOnly),
    /// Delete a document and all its contents
    DocumentDrop(DocumentOnly),
    ///Insert a field into a document
    FieldInsert(FieldWithData),
    /// Read contents particular document
    FieldRead(FieldNoData),
    /// Remove a particular document
    FieldRemove(FieldNoData),
    /// Updata a document
    FieldModifyDocument(FieldWithData),
}

type Key = String;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Permissions {
    SuperUser(Key),
    PrivilegedUser(Key),
    UnprivilegedUser(Key),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TuringHeaders {
    Terminator,
    Initializer
}
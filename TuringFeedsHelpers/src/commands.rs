use serde::{Deserialize, Serialize};
use crate::{DocumentOnly, FieldWithData, FieldNoData};

/// Commands to perform on the repo and its contents by the repo owner known as `SuperUser`
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TuringCommands {
    /// Initialize the Repository
    RepoCreate,
    /// Delete the Repository
    RepoDrop,
    /// Create a database
    DbCreate(String),
    /// Read documents in a database
    DbRead(String),
    /// List all databases in a repo
    DbList(String),
    /// Delete a database
    DbDrop(String),
    /// Create a document
    DocumentCreate(DocumentOnly),
    /// List all fields in a document
    DocumentRead(DocumentOnly),
    /// Delete a document and all its contents
    DocumentDrop(DocumentOnly),
    ///Insert a field into a document
    FieldInsert(FieldWithData),
    /// Read contents particular document
    FieldRead(FieldNoData),
    /// Remove a particular document
    FieldRemove(FieldNoData),
    /// Updata a document
    FieldModify(FieldWithData),
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
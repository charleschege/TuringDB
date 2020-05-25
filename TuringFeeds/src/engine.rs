use custom_codes::{DbOps, FileOps};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};
use tai64::TAI64N;
use lazy_static::*;
use anyhow::Result;
use evmap::{ReadHandle, WriteHandle};

use turingfeeds_helpers::{DocumentOnly, FieldNoData, FieldWithData};


#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Tdb {
    datetime: TAI64N,
    list: HashMap<SledDocumentName, Document>,
    //rights: Option<HashMap<UserIdentifier, (Role, AccessRights)>>,
    //database_hash: Blake2hash,
    //secrecy: TuringSecrecy,
    //config: TuringConfig,
    //authstate: Assymetric Crypto
    //superuser: Only one
    // admins: vec![], -> (User, PriveledgeAccess)
    //users: vec![] -> """"
}

impl Hash for Tdb {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.datetime.hash(state);

        for (key, value) in self.list.iter() {
            key.hash(state);
            value.hash(state);
        }
    }
}

impl Tdb {
    async fn new() -> Self {
        Self {
            datetime: TAI64N::now(),
            list: HashMap::new(),
        }
    }
}

type SledDocumentName = String;
type FieldName = String;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
struct Document {
    list: HashMap<FieldName, FieldMetadata>,
    //create_time: TAI64N,
    //modified_time: TAI64N,
}

impl Hash for Document {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (key, value) in self.list.iter() {
            key.hash(state);
            value.hash(state);
        }
    }
}

impl Document {
    async fn new() -> Self {
        Self { list: HashMap::new() }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
struct FieldMetadata {  
    create_time: TAI64N,
    modified_time: TAI64N,
    //data: Vec<u8>, // This cant go to in-memory DB because of size
    //primary_key: Option<UserDefinedName>,
    //indexes: Vec<String>,
    //hash: SeaHashCipher,
    //structure: Structure,
}

impl FieldMetadata {
    async fn new() -> Self {
        let now = TAI64N::now();

        Self {
            create_time: now,
            modified_time: now,
        }
    }
}

// Get structure from file instead of making it a `pub` type
#[allow(unused_variables)]
#[derive(Debug, Serialize, Deserialize)]
enum Structure {
    Schemaless,
    Schema,
    Vector,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum DocumentRights {
    /// Create Access
    C,
    /// Read Access
    R,
    /// Write Access
    W,
    /// Delete Access
    D,
    /// Forward
    F,
    /// Create Read Write Delete Access
    CRWD,
    /// Read Write Access
    RW,
}

#[allow(dead_code)]
enum TuringConfig {
    DefaultCOnfig,
    WriteACKs,
}
// Shows the level of security from the database level to a document level
#[allow(dead_code)]
enum TuringSecrecy {
    DatabaseMode,
    TableMode,
    DocumentMode,
    DefaultMode,
    InactiveMode,
}

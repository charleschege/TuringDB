use custom_codes::{DbOps, FileOps};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{DirBuilder, OpenOptions, self},
    path::{Path, PathBuf},
};
use tai64::TAI64N;
use anyhow::Result;
use async_lock::Lock;
use smol::{blocking, reader};
use futures::prelude::*;
#[derive(Debug, Clone)]
pub struct TuringEngine<'tf> {
    dbs: HashMap<&'tf str, Lock<Tdb<'tf>>>,
    datetime: TAI64N,
}


// READ
// UPDATE
// DELETE

impl<'tf> TuringEngine<'tf> {
    /// Create a new in-memory repo
    pub async fn new() -> TuringEngine<'tf> {
        Self {
            dbs: HashMap::new(),
            datetime: TAI64N::now(),
        }
    }
    /// Create a repo
    pub async fn create_repo(&self) -> Result<DbOps> {
        let path = "TuringFeedsRepo";
        blocking!(DirBuilder::new()
            .recursive(false)
            .create(path))?;
        
        self.create_ops_log_file().await?;
        self.create_errors_log_file().await?;

        Ok(DbOps::RepoCreated)
    }
    /// Create a new repository/directory that contains the databases
    async fn create_ops_log_file(&self) -> Result<()> {
        blocking!(OpenOptions::new()
            .create_new(true)
            .write(true)
            .open("TuringFeedsRepo/ops.log"))?;
        Ok(())
    }
    /// Create a new repository/directory that contains the databases
    async fn create_errors_log_file(&self) -> Result<()> {
        blocking!(OpenOptions::new()
            .create_new(true)
            .write(true)
            .open("TuringFeedsRepo/errors.log"));
        
        Ok(())
    }
    /// Check if the repository is empty
    pub async fn is_empty(&self) -> bool {
        self.dbs.is_empty()
    }
    /*/// Read a repo
    pub async fn init(&mut self) -> Result<&TuringEngine<'tf>> {
        let mut repo = blocking!(fs::read_dir("TuringFeedsRepo"))?;
        while let Some(entry) = repo.next() {
            let entry = entry?;
            let inner = entry.file_type()?;
            let db_path = entry.path().clone().as_path();

            if inner.is_dir() == true {
                self.load_tdb(db_path).await;
            }
        }

        Ok(self)
    }
    /// Load the contents of the log file of a database into memory
    async fn load_tdb(&mut self, db_path: &'tf Path) -> Result<&TuringEngine<'tf>>{               
        let mut contents: Vec<u8> = Vec::new();
        if let Some(file_name) = db_path.file_name() {
            let mut metadata: PathBuf = db_path.into();
            metadata.push(file_name);
            metadata.set_extension("log");

            let file = blocking!(OpenOptions::new()
                .create(false)
                .read(true)
                .write(false)
                .open(metadata))?;
            let mut file = reader(file);

            file.read(&mut contents).await?;
            let data = bincode::deserialize::<Tdb>(&contents)?;
            //self.dbs.insert(file_name.clone().to_string_lossy(), Lock::new(data));
        }

        Ok(self)
    }*/
} 



#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tdb<'tf> {
    datetime: TAI64N,
    list: HashMap<&'tf str, Document<'tf>>,
    //rights: Option<HashMap<UserIdentifier, (Role, AccessRights)>>,
    //database_hash: Blake2hash,
    //secrecy: TuringSecrecy,
    //config: TuringConfig,
    //authstate: Assymetric Crypto
    //superuser: Only one
    // admins: vec![], -> (User, PriveledgeAccess)
    //users: vec![] -> """"
}

impl<'tf> Tdb<'tf> {
    async fn new() -> Tdb<'tf> {
        Self {
            datetime: TAI64N::now(),
            list: HashMap::new(),
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
struct Document<'tf> {
    list: HashMap<&'tf str, FieldMetadata>,
    create_time: TAI64N,
}

impl<'tf> Document<'tf> {
    async fn new() -> Document<'tf> {
        Self { list: HashMap::new(), create_time: TAI64N::now() }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct FieldMetadata {  
    create_time: TAI64N,
    modified_time: TAI64N,
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
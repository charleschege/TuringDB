use async_std::{
    fs,
    fs::{DirBuilder, File, OpenOptions},
    io::{prelude::*, BufReader, ErrorKind, Seek, SeekFrom},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    sync::Arc,
    task,
    stream::StreamExt,
};
use custom_codes::{DbOps, FileOps};
use serde::{Deserialize, Serialize};
use turingfeeds_helpers::TuringFeedsError;
use std::{
    io::Read,
    collections::HashMap,
};
use tai64::TAI64N;
use dashmap::DashMap;

use crate::{AccessRights, RandIdentifier, Result, Role,};

/// No need for rights as the user who decrypts the DB has total access
/*
#[derive(Debug)]
pub struct TuringFeeds {
    dbs: Arc<DashMap<String, TuringFeedsDB>>,
    //hash: RepoBlake2hash,
    //secrecy: TuringSecrecy,
    //config: TuringConfig,
    //authstate: Assymetric Crypto
    //superuser: Only one
    // admins: vec![], -> (User, PriveledgeAccess)
    //users: vec![] -> ""
}

impl TuringFeeds {
    /// Initialize the structure with default values
    pub async fn new() -> Self {
        Self {
            dbs: Arc::new(DashMap::default()),
        }
    }
    /// Recursively walk through the Directory
    /// Load all the Directories into memory
    /// Hash and Compare with Persisted Hash to check for corruption
    /// Throw errors if any otherwise
    pub async fn init(&self) -> Result<TuringFeeds> {
        let mut repo_path = PathBuf::new();
        repo_path.push("TuringFeedsRepo");
        repo_path.push("REPO");
        repo_path.set_extension("log");

        let mut contents = String::new();
        let mut file = OpenOptions::new()
            .create(false)
            .read(true)
            .write(true)
            .open(repo_path)
            .await?;

        file.read_to_string(&mut contents).await?;
        let data = ron::de::from_str::<DashMap<String, TuringFeedsDB>>(&contents)?;

        let data = Self { dbs: Arc::new(data) };

        Ok(data)
    }
    /// Create a new repository/directory that contains the databases
    pub async fn create(&self) -> Result<FileOps> {
        let mut repo_path = PathBuf::new();
        repo_path.push("TuringFeedsRepo");

        match DirBuilder::new().recursive(false).create(repo_path).await {
            Ok(_) => Ok(FileOps::CreateTrue),
            Err(error) => Err(TuringFeedsError::IoError(error)),
        }
    }
    /// Create a new repository/directory that contains the databases
    pub async fn drop_repo(&self) -> Result<FileOps> {
        let mut repo_path = PathBuf::new();
        repo_path.push("TuringFeedsRepo");

        match fs::remove_dir(repo_path).await {
            Ok(_) => Ok(FileOps::DeleteTrue),
            Err(error) => Err(TuringFeedsError::IoError(error)),
        }
    }
    /// Create the Metadata file or add data to the metadata file
    pub async fn commit(&self) -> Result<FileOps> {
        let mut repo_path = PathBuf::new();
        repo_path.push("TuringFeedsRepo");
        repo_path.push("REPO");
        repo_path.set_extension("log");

        match OpenOptions::new()
            .create(true)
            .read(false)
            .write(true)
            .open(repo_path)
            .await
        {
            Ok(mut file) => {
                let data = ron::ser::to_string(&*self.dbs)?;
                file.write_all(&data.as_bytes().to_owned()).await?;
                file.sync_all().await?;

                Ok(FileOps::WriteTrue)
            }
            Err(error) => Err(TuringFeedsError::IoError(error)),
        }
    }
    /// Add a Database
    pub async fn memdb_add(&self, values: TuringFeedsDB) -> Result<(DbOps, Option<&Self>)> {
        if self.dbs.contains_key(&values.identifier) == true {
            Ok((DbOps::AlreadyExists, None))
        }else {
            match self.create_disk_db(&values.identifier).await {
                Ok(_) => {
                    self.dbs.insert(values.identifier.clone(), values);

                    Ok((DbOps::DbCreated, Some(self)))
                },
                Err(error) => Err(error),
            }
        }
    }
    /// List all databases on disk in the current repository
    pub async fn list_memdbs(&self) -> Vec<String> {
        let dbs = &self.dbs;        
        let mut list = Vec::new();

        for inner_data in dbs.iter() {
            list.push(inner_data.key().to_owned());
        }

        list
    }
    /// Remove a Database if it exists
    pub async fn memdb_rm(&self, db_name: &str) -> Result<DbOps> {
        if self.dbs.contains_key(db_name) == true {
            Ok(DbOps::AlreadyExists)
        }else {
            match self.rm_disk_db(db_name).await {
                Ok(_) => {                    
                    if let Some(_) = self.dbs.remove(db_name) {
                        Ok(DbOps::DbDropped)
                    }else {
                        Ok(DbOps::DbNotFound)
                    }
                },
                Err(error) => Err(error),
            }
        }
    }
    /// Create a database directory on disk to house tables
    async fn create_disk_db(&self, db_name: &str) -> Result<()> {
        let mut db_path = PathBuf::new();
        db_path.push("TuringFeedsRepo");
        db_path.push(db_name);

        Ok(DirBuilder::new()
            .recursive(false)
            .create(db_path)
            .await?)
    }
    /// Remove a database from disk
    async fn rm_disk_db(&self, db_name: &str) -> Result<()> {
        let mut db_path = PathBuf::new();
        db_path.push("TuringFeedsRepo");
        db_path.push(db_name);

        Ok(fs::remove_dir(db_path).await?)
    }
    /// List all databases on disk in the current repository
    pub async fn list_dbs_on_disk(&self) -> Result<Vec<String>> {
        let mut repo_path = PathBuf::new();
        repo_path.push("TuringFeedsRepo");

        let mut dbs = fs::read_dir(repo_path).await?;
        let mut list = Vec::new();
        while let Some(entry) = dbs.next().await {
            let entry = entry?;
            list.push(entry.file_name().to_os_string().into_string()?)
        }

        Ok(list)
    }
    /// Deal with documents
    ///
    /// Create an in-memory document
    pub async fn memdb_doc_create(&self, db: &str, doc_identifier: &str, doc_data: TFDocument) {
        match self.dbs.get_mut(db) {
            Some(mut memdb) => {
                println!("AT SELECT DB");
                
                if let Some(doc_list) = &memdb.document_list {
                    if doc_list.contains_key(doc_identifier) == true {
                        dbg!(DbOps::DocumentFound);
                    }else {
                        doc_list.insert(doc_identifier.to_owned(), doc_data);
                        println!("WHEN DOCS IS SOME--------");
                        dbg!(&*memdb);
                    }
                }else {
                    memdb.document_list = {
                        let data: DashMap<String, TFDocument> = DashMap::new();
                        data.insert(doc_identifier.to_owned(), doc_data);

                        Some(data)
                    };
                    println!("WHEN DOCS IS NONE--------");
                    dbg!(&*memdb);
                }
                println!("------\n");
            },
            None => { dbg!(DbOps::DbNotFound); }
        }
    }
    
    /*/// Create an empty document
    pub async fn create_doc(&self, db: &str, doc: &TFDocument) -> Result<DbOps>{
        // Get db name
        // check if doc already exists in inmemory database
        // add or reject

        let mut document_path = PathBuf::new();
        document_path.push("TuringFeedsRepo");
        document_path.push(db);
        document_path.push(&doc.identifier);
        
        match self.dbs.get(db) {
            Some(value) => {      
                // Check whether DB is empty          
                if let Some(document) = &value.document_list {
                    //if db is not empty check whether the field exists
                    if document.contains_key(&doc.identifier) == true {
                        Ok(DbOps::AlreadyExists)
                    }else {
                        match sled::Config::default()
                            .create_new(true)
                            .path(document_path)
                            .open() {
                                Err(sled::Error::CollectionNotFound(_)) => Ok(DbOps::DocumentNotFound),
                                Err(sled::Error::Io(inner)) => {
                                    match inner {
                                        match inner.kind() with the TuringHelpers crate
                                    }
                                }
                                Err(sled_error) => Err(sled_errors(sled_error).await),
                                Ok(_) => {
                                    self.dbs.get_mut(db).insert()
                                    Ok(DbOps::DocumentCreated)
                                },
                            }
                    }
                }else {
                    // If the DB is empty insert a new document
                    match sled::Config::default()
                        .create_new(false)
                        .path(document_path)
                        .open() {
                            Err(sled::Error::CollectionNotFound(_)) => Ok(DbOps::DocumentNotFound),
                            Err(sled_error) => Err(sled_errors(sled_error).await),
                            Ok(_) => Ok(DbOps::DocumentCreated),
                        }
                }
            },
            None => Ok(DbOps::DbNotFound),
        }
    }*/
    // TODO create a database Dir on disk and modify inmemory Map
    // TODO remove a database Dir from disk and modify inmemory Map
    // TODO Clear all databases from disk after authentication and thereafter dropping it from in-memory map
}

async fn sled_errors(error: sled::Error) -> TuringFeedsError {
    use sled::Error as SledError;
    use async_std::io::Error;
    match error {
        SledError::CollectionNotFound(_) => TuringFeedsError::IoError(Error::new(ErrorKind::Other, "SledError::CollectionNotFound")),
        SledError::Unsupported(value) => TuringFeedsError::IoError(Error::new(ErrorKind::Other, "SledError::".to_owned() + value.as_str())),
        SledError::ReportableBug(value) => TuringFeedsError::IoError(Error::new(ErrorKind::Other, "SledError::".to_owned() + value.as_str())),
        SledError::Io(value) => TuringFeedsError::IoError(Error::new(ErrorKind::Other, value)),
        SledError::Corruption{at} => TuringFeedsError::IoError(Error::new(ErrorKind::Other, format!("SledError::{:?}", at))),
    }
}

type DocumentName = String;
type FieldIdentifier = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct TuringFeedsDB {
    identifier: String,
    datetime: TAI64N,
    document_list: Option<DashMap<String, DashMap<FieldIdentifier, FieldMetadata>>>,
    //rights: Option<HashMap<UserIdentifier, (Role, AccessRights)>>,
    //database_hash: Blake2hash,
    //secrecy: TuringSecrecy,
    //config: TuringConfig,
    //authstate: Assymetric Crypto
    //superuser: Only one
    // admins: vec![], -> (User, PriveledgeAccess)
    //users: vec![] -> """"
}

impl TuringFeedsDB {
    pub async fn new() -> Self {
        Self {
            identifier: String::default(),
            datetime: TAI64N::now(),
            document_list: Option::default(),
        }
    }
    pub async fn identifier(mut self, key: &str) -> Self {
        self.identifier = key.to_owned();

        self
    }
    pub async fn add(mut self, identifier: &str, values: TFDocument) -> Self {
        if let Some(mut existing_map) = self.document_list {
            match existing_map.insert(identifier.to_owned(), values) {
                Some(_) => {
                    // If the value existed in the map
                    self.datetime = TAI64N::now();
                    self.document_list = Some(existing_map);

                    self
                }
                None => {
                    self.datetime = TAI64N::now();
                    self.document_list = Some(existing_map);

                    self
                }
            }
        } else {
            let mut new_map = DashMap::new();
            new_map.insert(identifier.to_owned(), values);
            self.datetime = TAI64N::now();
            self.document_list = Some(new_map);

            self
        }
    }
    pub async fn rm(mut self, key: &str) -> (DbOps, Self) {
        if let Some(mut existing_map) = self.document_list {
            match existing_map.remove(key) {
                Some(_) => {
                    // If the value existed in the map
                    self.datetime = TAI64N::now();
                    self.document_list = Some(existing_map);
                    (DbOps::Deleted, self)
                }
                None => {
                    // If the key does not exist in the map
                    self.document_list = Some(existing_map);
                    (DbOps::KeyNotFound, self)
                }
            }
        } else {
            // The Repository does not have any databases
            (DbOps::Empty, self)
        }
    }
}
*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FieldMetadata {  
    create_time: TAI64N,
    modified_time: TAI64N,
    //data: Vec<u8>, // This cant go to in-memory DB because of size
    //primary_key: Option<UserDefinedName>,
    //indexes: Vec<String>,
    //hash: SeaHashCipher,
    //structure: Structure,
}

impl FieldMetadata {
    pub async fn new() -> Self {
        let now = TAI64N::now();

        Self {
            create_time: now,
            modified_time: now,
        }
    }

    pub async fn swap(&mut self, value: FieldMetadata) -> &Self {
        self.create_time = value.create_time;
        self.modified_time = value.modified_time;

        self
    }

    pub async fn update_modified_time(mut self) -> Self {
        self.modified_time = TAI64N::now();

        self
    }

    pub async fn get_create_time(&self) -> TAI64N {
        self.create_time
    }

    pub async fn get_modified_time(&self) -> TAI64N {
        self.modified_time
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

enum TuringConfig {
    DefaultCOnfig,
    WriteACKs,
}
// Shows the level of security from the database level to a document level
enum TuringSecrecy {
    DatabaseMode,
    TableMode,
    DocumentMode,
    DefaultMode,
    InactiveMode,
}

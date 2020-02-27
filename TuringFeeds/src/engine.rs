use async_std::{
    fs,
    fs::{DirBuilder, File, OpenOptions, ReadDir},
    io::{prelude::*, BufReader, ErrorKind, Seek, SeekFrom},
    net::{TcpListener, TcpStream},
    path::{PathBuf, Path},
    sync::{Arc, Mutex},
    task,
    stream::StreamExt,
};
use custom_codes::{DbOps, FileOps};
use serde::{Deserialize, Serialize};
use std::{
    io::Read,
    collections::HashMap,
    hash::{Hash, Hasher},
};
use tai64::TAI64N;
use lazy_static::*;
use sled::{TransactionResult, Config, IVec};
use turingfeeds_helpers::{TuringFeedsError, DocumentMethods};

use crate::{AccessRights, RandIdentifier, Result, Role};
use lazy_static::*;
use evmap::{ReadHandle, WriteHandle};

/// No need for rights as the user who decrypts the DB has total access

/*
lazy_static!{
    static ref DB: (Arc<Mutex<ReadHandle<&'static str, Box<Bar>>>>, Arc<Mutex<WriteHandle<&'static str, Box<Bar>>>>) = {
        let (reader, writer) = evmap::new();

        (Arc::new(Mutex::new(reader)), Arc::new(Mutex::new(writer)))
    };

    //These are just for convinience
    static ref READER: Arc<Mutex<ReadHandle<&'static str, Box<Bar>>>> = DB.0.clone();
    static ref WRITER: Arc<Mutex<WriteHandle<&'static str, Box<Bar>>>> = DB.1.clone();
}
*/
type EvmapReader = ReadHandle<String, Box<Tdb>>;
type EvmapWriter = Arc<Mutex<WriteHandle<String, Box<Tdb>>>>;

/// Handle list of databases
#[derive(Debug)]
pub struct TuringFeeds {
    //reader: Arc<Mutex<ReadHandle<String, Box<Tdb>>>>,
    reader: ReadHandle<String, Box<Tdb>>,
    writer: Arc<Mutex<WriteHandle<String, Box<Tdb>>>>,
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
        let dbs: (ReadHandle<String, Box<Tdb>>, WriteHandle<String, Box<Tdb>>) = evmap::new();

        Self {
            reader: dbs.0,
            writer: Arc::new(Mutex::new(dbs.1)),
        }
    }     
    /// Check whether the list are empty or not
    pub async fn repo_is_empty(&self) -> bool {
        
        self.reader.is_empty()
    }
    /// Recursively walk through the Directory
    /// Load all the Directories into memory
    /// Hash and Compare with Persisted Hash to check for corruption
    /// Throw errors if any otherwise    
    pub async fn repo_init(&mut self) -> &Self{
        let mut repo_path = PathBuf::new();
        repo_path.push("TuringFeedsRepo");
        repo_path.push("REPO");
        repo_path.set_extension("log");

        match fs::read_dir("TuringFeedsRepo").await {
            Ok(mut entries) => {
                while let Some(entry_found) = entries.next().await {
                    match entry_found {
                        Ok(entry) => {
                            match entry.file_type().await {
                                Ok(inner) => {
                                    if inner.is_dir() == true {
                                        //println!("{}", entry.path().to_string_lossy());
                                        self.load_tdb(entry.path()).await;
                                    }else {
                                        // Customize this to detect `error.log` and `ops.log` file
                                        println!("[TAI64N::<{:?}>] - [Tdb::<WARNING - FOUND A FILE `{}`>]", TAI64N::now(), entry.file_name().to_string_lossy());
                                    }
                                },
                                Err(error) => {
                                    eprintln!("[TAI64N::<{:?}>] - [Tdb::<ERROR READING `async_std::fs::FileType`>] - [ErrorKind - {:?}]", TAI64N::now(), error.kind());
                                    std::process::exit(1);
                                },
                            }
                        },
                        Err(error) => {
                            eprintln!("[TAI64N::<{:?}>] - [Tdb::<ERROR GETTING `async_std::fs::DirEntry`>] - [ErrorKind - {:?}]", TAI64N::now(), error.kind());
                            std::process::exit(1);
                        },
                    }                 
                }

                self
            },
            Err(error) => {
                eprintln!("[TAI64N::<{:?}>] - [Tdb::<ERROR READING `TuringFeedsRepo` DIRECTORY>] - [ErrorKind - {:?}]", TAI64N::now(), error.kind());
                std::process::exit(1);
            },
        }    
    }
    async fn load_tdb(&self, db_path: PathBuf) -> &Self{        
        let mut contents = String::new();
        if let Some(file_name) = db_path.file_name() {
            let mut metadata = db_path.clone();
            metadata.push(file_name);
            metadata.set_extension("log");

            match OpenOptions::new()
                .create(false)
                .read(true)
                .write(false)
                .open(metadata)
                .await 
                {
                    Ok(mut file) => {
                        println!("[TAI64N::<{:?}>] - [Tdb::<OPENING REPO METADATA FILE SUCCESSFUL>]", TAI64N::now());
                        match file.read_to_string(&mut contents).await {
                            Ok(_) => {
                                println!("[TAI64N::<{:?}>] - [Tdb::<READING REPO METADATA FILE COMPLETE>]", TAI64N::now());
                                match ron::de::from_str::<Tdb>(&contents) {
                                    Ok(data) => {
                                        println!("[TAI64N::<{:?}>] - [Tdb::<INITIALIZATION SUCCESSFUL>]", TAI64N::now());
                                        self.writer.lock().await.insert(file_name.clone().to_string_lossy().into(), Box::new(data));
                                        self.writer.lock().await.refresh();

                                        self
                                    },
                                    Err(error) => {
                                        eprintln!("[TAI64N::<{:?}>] - [Tdb::<RON/SERDE DESERIALIZATION ERROR>] - [SerdeError - {:?}]", TAI64N::now(), error);
                                        std::process::exit(1);
                                    }
                                }
                            },
                            Err(error) => {
                                eprintln!("[TAI64N::<{:?}>] - [Tdb::<ERROR READING REPO METADATA>] - [ErrorKind - {:?}]", TAI64N::now(), error.kind());
                                std::process::exit(1);
                            }
                        }
                    },
                    Err(error) => {
                        eprintln!("[TAI64N::<{:?}>] - [Tdb::<ERROR OPENING `{:?}`>] - [ErrorKind - {:?}]", TAI64N::now(), file_name, error.kind());
                        self
                    }
                }
        }else {
            eprintln!("[Tdb::<ERROR GETTING DB NAME `{:#?}`>]", db_path);
            std::process::exit(1);
        }
    }
    /// Create a new repository/directory that contains the databases
    pub async fn repo_create(&self) -> Result<FileOps> {
        let mut repo_path = PathBuf::new();
        repo_path.push("TuringFeedsRepo");

        match DirBuilder::new().recursive(false).create(repo_path).await {
            Ok(_) => {
                self.create_ops_log_file().await?;
                self.create_errors_log_file().await?;
                Ok(FileOps::CreateTrue)
            },
            Err(error) => Err(TuringFeedsError::IoError(error)),
        }
    }
    /// Create a new repository/directory that contains the databases
    async fn create_ops_log_file(&self) -> Result<FileOps> {
        let mut log_file_path = PathBuf::new();
        log_file_path.push("TuringFeedsRepo");
        log_file_path.push("ops.log");

        match OpenOptions::new()
            .create(true)
            .write(true)
            .open(log_file_path)
            .await {
                Ok(_) => Ok(FileOps::CreateTrue),
                Err(error) => Err(TuringFeedsError::IoError(error)),
            }
    }
    /// Create a new repository/directory that contains the databases
    async fn create_errors_log_file(&self) -> Result<FileOps> {
        let mut log_file_path = PathBuf::new();
        log_file_path.push("TuringFeedsRepo");
        log_file_path.push("errors.log");

        match OpenOptions::new()
            .create(true)
            .write(true)
            .open(log_file_path)
            .await {
                Ok(_) => Ok(FileOps::CreateTrue),
                Err(error) => Err(TuringFeedsError::IoError(error)),
            }
    }
    /// Create a new repository/directory that contains the databases
    pub async fn repo_drop(&self) -> Result<FileOps> {
        // TODO - list all the databases and their fields that are being dropped and log to `ops.log` file
        let mut repo_path = PathBuf::new();
        repo_path.push("TuringFeedsRepo");

        match fs::remove_dir_all(repo_path).await {
            Ok(_) => Ok(FileOps::DeleteTrue),
            Err(error) => Err(TuringFeedsError::IoError(error)),
        }
    }
    // TODO DB - CRUDL
    // TODO DOCUMENT - CRUDL
    // TODO FIELD - CRUDL

    // DONE! CREATE
    // TODO GET DATABASE FIELDS --- CHECK
    // DONE! LIST DATABASES
    // DONE! DELETE

    /// Add a Database
    pub async fn db_create(&self, db_name: &str) -> Result<DbOps> {
        if self.reader.contains_key(db_name) == true {
            Ok(DbOps::DbAlreadyExists)
        }else {
            let values = Tdb::new().await;
            values.create_on_disk(db_name).await?;
            
            self.writer.lock().await.insert(db_name.into(), Box::new(values.get_self().await));
            self.writer.lock().await.refresh();
            Ok(DbOps::DbCreated)
        }
    }
    /// Remove a Database if it exists
    pub async fn db_drop(&self, db_name: &str) -> Result<DbOps> {
        if let Some(db) = self.db_get(db_name).await {
            let mut values = Tdb::new().await;
            values.swap(&db).await;
            values.disk_db_drop(db_name).await?;
            self.writer.lock().await.empty(db_name.into());
            self.writer.lock().await.refresh();
            Ok(DbOps::DbDropped)
        }else {
            Ok(DbOps::DbNotFound)
        }
    }
    /// Get the documents of a database
    pub async fn db_read(&self, db_name: &str) -> DbOps {
        if let Some(db) = self.db_get(db_name).await {
            let mut data: Vec<String> = Vec::new();

            for key in db.list.keys() {
                data.push(key.into());
            }
            if data.is_empty() == true {
                DbOps::DbEmpty
            }else {
                DbOps::DocumentList(data)
            }
        }else {
            DbOps::DbNotFound
        }
    }
    /// Get list of databases
    pub async fn db_list(&self) -> DbOps {
        let mut db_list: Vec<String> = Vec::new();
        for (key, _) in &self.reader.read() {
            db_list.push(key.into());
        }

        if db_list.is_empty() == true {
            DbOps::RepoEmpty
        }else {
            DbOps::DbList(db_list)
        }
    }
    /// Get a Database if it exists. This function is not public and is used by methods in this `impl TuringFeeds` block
    /// to get the documents in a given database
    async fn db_get(&self, db_name: &str) -> Option<Tdb> {
        match self.reader.get(db_name) {
            Some(value) => { 
                if let Some(value) = value.iter().next().clone() {
                    Some(*value.clone())
                }else {
                    None
                }
             },
            None => { None }
        }
    }
    // ******DOCUMENTS************
    /// TODO CREATE DOCUMENT & UPDATE LOG
    /// TODO READ A DOCUMENT
    /// TODO LIST ALL DOCUMENTs
    /// TODO UPDATE A DOCUMENT & UPDATE LOG
    /// TODO DELETE A DOCUMENT & UPDATE LOG
    
    /// Update a field 
    pub async fn document_create(&mut self, db_name: &str, doc_name: &str) -> Result<DbOps> {
        let mut doc_path = PathBuf::new();
        doc_path.push("TuringFeedsRepo");
        doc_path.push(db_name);
        doc_path.push(doc_name);

        if let Some(mut db) = self.db_get(&db_name).await {
            if db.list.contains_key(doc_name) == true {
                Ok(DbOps::DocumentAlreadyExists)
            }else {
                dbg!(&doc_path);
                match sled::Config::default()
                    .create_new(false)
                    .path(doc_path)
                    .open() {
                        Ok(_) => {
                            db.insert(doc_name.into(), Documents::new().await).await?;
                            db.commit(db_name).await?;
                            self.writer.lock().await.update(db_name.into(), Box::new(db));
                            self.writer.lock().await.refresh();

                            Ok(DbOps::DocumentCreated)
                        },
                        Err(error) => Err(sled_errors(error).await),
                    }
            }
        }else {
            Ok(DbOps::DbNotFound)
        }
    }
}

#[derive(Debug)]
struct LogFile {
    error: TuringFeedsError,
    timestamp: TAI64N,
}

impl LogFile {
    ///Write to logger file
    async fn log_error_to_file(&self, error: TuringFeedsError) -> Result<()> {
        let mut log_file_path = PathBuf::new();
        log_file_path.push("TuringFeedsRepo");
        log_file_path.push("errors.log");

        let mut file = OpenOptions::new()
            .create(false)
            .read(false)
            .append(true)
            .open(log_file_path)
            .await?;
        let error_customized = format!("[TAI64N - {:?}] <-> {:?}", TAI64N::now(), error);

        file.write_all(&error_customized.into_bytes()).await?;
        Ok(file.sync_all().await?)
    }
}

async fn sled_errors(error: sled::Error) -> TuringFeedsError {
    use sled::Error as SledError;
    use async_std::io::Error;
    match error {
        SledError::CollectionNotFound(_) => TuringFeedsError::IoError(Error::new(ErrorKind::Other, "SledError::CollectionNotFound")),
        SledError::Unsupported(value) => TuringFeedsError::IoError(Error::new(ErrorKind::Other, "SledError::".to_owned() + value.as_str())),
        SledError::ReportableBug(value) => TuringFeedsError::IoError(Error::new(ErrorKind::Other, "SledError::".to_owned() + value.as_str())),
        SledError::Io(value) => TuringFeedsError::IoError(value),
        SledError::Corruption{at} => TuringFeedsError::IoError(Error::new(ErrorKind::Other, format!("SledError::{:?}", at))),
    }
}
type DocumentsID = String;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
struct Tdb {
    datetime: TAI64N,
    list: HashMap<DocumentsID, Documents>,
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
    async fn get_self(self) -> Self {
        self
    }   
    /// Check whether the list are empty or not
    async fn is_empty(&self) -> bool {
        
        self.list.is_empty()
    }

    async fn insert(&mut self, identifier: &str, values: Documents) -> Result<DbOps> {

        if self.list.contains_key(identifier) == true {
            Ok(DbOps::DocumentAlreadyExists)
        }else {            
            self.list.insert(identifier.into(), values);
            Ok(DbOps::DocumentCreated)
        }
    }

    /// Create a new database that contains the databases
    async fn create_on_disk(&self, identifier: &str) -> Result<FileOps> {
        let mut db_path = PathBuf::new();
        db_path.push("TuringFeedsRepo");
        db_path.push(identifier);
        

        match DirBuilder::new().recursive(false).create(db_path).await {
            Ok(_) => {
                self.commit(identifier).await
            },
            Err(error) => Err(turingfeeds_helpers::TuringFeedsError::IoError(error)),
        }
    }
    /// Create the Metadata file or add data to the metadata file
    async fn commit(&self, identifier: &str) -> Result<FileOps> {
        let mut db_path = PathBuf::new();
        db_path.push("TuringFeedsRepo");
        db_path.push(identifier);
        db_path.push(identifier);
        db_path.set_extension("log");

        match OpenOptions::new()
            .create(true)
            .read(false)
            .write(true)
            .open(db_path)
            .await
        {
            Ok(mut file) => {
                let data = ron::ser::to_string(&self)?;
                file.write_all(&data.as_bytes().to_owned()).await?;
                file.sync_all().await?;

                Ok(FileOps::WriteTrue)
            }
            Err(error) => Err(TuringFeedsError::IoError(error)),
        }
    } 

    async fn get(&self, identifier: &str) -> Option<&Documents> {
        self.list.get(identifier)
    }
    async fn list_docs(&self) -> Vec<String> {
        let mut documents: Vec<String> = Vec::new();

        for key in self.list.keys() {
            documents.push(key.into())
        }

        documents
    }

    async fn update(&mut self, identifier: &str, values: Documents) -> DbOps {
        if self.list.contains_key(identifier) == true {
            self.list.insert(identifier.into(), values);

            DbOps::DbModified
        }else {
            DbOps::DbNotFound
        }
    }

    async fn swap(&mut self, values: &Tdb) -> &Self {
        self.datetime = values.datetime.clone();
        self.list = values.list.clone();

        self
    }
    
    /// Create a new repository/directory that contains the databases
    async fn disk_db_drop(&self, identifier: &str) -> Result<FileOps> {
        let mut db_path = PathBuf::new();
        db_path.push("TuringFeedsRepo");
        db_path.push(identifier);

        match fs::remove_dir_all(db_path).await {
            Ok(_) => Ok(FileOps::DeleteTrue),
            Err(error) => Err(TuringFeedsError::IoError(error)),
        }
    }
}

type SledDocumentName = String;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
struct Documents {
    list: HashMap<SledDocumentName, Fields>,
    //create_time: TAI64N,
    //modified_time: TAI64N,
}

impl Hash for Documents {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (key, value) in self.list.iter() {
            key.hash(state);
            value.hash(state);
        }
    }
}

// TODO CRUD
// TODO Sled CRUD

impl Documents {
    async fn new() -> Self {
        Self { list: HashMap::new() }
    }    
    /// Check whether the list are empty or not
    async fn is_empty(&self) -> bool {
        
        self.list.is_empty()
    }

    async fn insert(&mut self, identifier: &str, values: Fields) -> DbOps {
        if self.list.contains_key(identifier) == true {
            DbOps::DocumentAlreadyExists
        }else {
            self.list.insert(identifier.into(), values);

            DbOps::DocumentInserted
        }
    }

    async fn get(&self, identifier: &str) -> Option<&Fields> {
        self.list.get(identifier)
    }

    async fn update(&mut self, identifier: &str, values: &Fields) -> DbOps {
        if self.list.contains_key(identifier) == true {
            self.list.insert(identifier.into(), values.clone());

            DbOps::DocumentModified
        }else {
            DbOps::DocumentNotFound
        }
    }

    async fn remove(&mut self, identifier: &str) -> DbOps {
        if let Some(_) = self.list.remove(identifier) {
            DbOps::DocumentDropped
        }else {
            DbOps::DocumentNotFound
        }
    }

    async fn swap(&mut self, value: &Documents) -> &Self {
        self.list = value.list.clone();
        

        self
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
struct Fields {
    // metadata includes timestamp for `Self` for each particular field
    list: HashMap<String, FieldMetadata>
}

impl Hash for Fields {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (key, value) in self.list.iter() {
            key.hash(state);
            value.hash(state);
        }
    }
}

impl Fields {
    async fn new() -> Self {

        Self {
            list: HashMap::default(),
        }
    }
    /// Check whether the fields are empty or not
    async fn is_empty(&self) -> bool {
        
        self.list.is_empty()
    }

    async fn insert(&mut self, identifier: &str, value: FieldMetadata) -> DbOps {
        
        if self.list.contains_key(identifier) == true { 

            DbOps::FieldAlreadyExists
        }else {
            self.list.insert(identifier.into(), value);

            drop(identifier);
            
            DbOps::FieldInserted
        }
    }
    /// Get the field returning (FieldName, FieldMetadata)
    async fn get(&self, identifier: &str) -> Option<&FieldMetadata> {

        match self.list.get(identifier) {
            Some(field) => Some(field),
            None => None,
        }
    }
    /// Get only the key since the value only updates the time modified 
    /// which is done automatically by the `FieldMetadata::new().await.update_modified_time().await` method
    async fn update(&mut self, identifier: &str) -> DbOps {

        match self.list.get_mut(identifier) {
            Some(field) => {
                field.update_modified_time().await;
                
                DbOps::FieldModified
            }
            None => DbOps::FieldNotFound,
        }
    }

    async fn remove(&mut self, identifier: &str) -> DbOps {

        if let Some(_) = self.list.remove(identifier) {
            DbOps::FieldDropped
        }else {
            DbOps::FieldNotFound
        }
    }

    async fn swap(&mut self, value: &Fields) -> &Self {
        self.list = value.list.clone();

        self
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

    async fn swap(&mut self, value: &FieldMetadata) -> &Self {
        self.create_time = value.create_time;
        self.modified_time = value.modified_time;

        self
    }

    async fn update_modified_time(&mut self) -> &Self {
        self.modified_time = TAI64N::now();

        self
    }

    async fn get_create_time(&self) -> TAI64N {
        self.create_time
    }

    async fn get_modified_time(&self) -> TAI64N {
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

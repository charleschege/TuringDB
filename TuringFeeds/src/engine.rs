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
use turingfeeds_helpers::TuringFeedsError;

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
type EvmapReader = Arc<Mutex<ReadHandle<String, Box<Tdb>>>>;
type EvmapWriter = Arc<Mutex<WriteHandle<String, Box<Tdb>>>>;
type DatabaseID = String;
/// Handle list of databases
#[derive(Debug)]
pub struct TuringFeeds {
    reader: Arc<Mutex<ReadHandle<String, Box<Tdb>>>>,
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
            reader: Arc::new(Mutex::new(dbs.0)),
            writer: Arc::new(Mutex::new(dbs.1)),
        }
    } 
    /// Initialize the structure with default values
    pub async fn swap(mut self, reader: EvmapReader, writer: EvmapWriter) -> Self {
        self.reader = reader;
        self.writer = writer;

        self
    }      
    /// Check whether the list are empty or not
    pub async fn repo_is_empty(&self) -> bool {
        
        self.reader.lock().await.is_empty()
    }
    /// Recursively walk through the Directory
    /// Load all the Directories into memory
    /// Hash and Compare with Persisted Hash to check for corruption
    /// Throw errors if any otherwise    
    pub async fn init(&mut self) -> &Self{
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
                                        self.insert_tdb(entry.path()).await;
                                    }else {
                                        println!("[Tdb::<WARNING - FOUND A FILE `{}` INSTEAD OF A DIRECTORY>]", entry.file_name().to_string_lossy());
                                        std::process::exit(1);
                                    }
                                },
                                Err(error) => {
                                    eprintln!("[Tdb::<ERROR READING `async_std::fs::FileType`>]\n     {}", error);
                                    std::process::exit(1);
                                },
                            }
                        },
                        Err(error) => {
                            eprintln!("[Tdb::<ERROR GETTING `async_std::fs::DirEntry`>]\n     {}", error);
                            std::process::exit(1);
                        },
                    }                 
                }

                self
            },
            Err(error) => {
                eprintln!("[Tdb::<ERROR READING `TuringFeedsRepo` DIRECTORY>]\n     {}", error);
                std::process::exit(1);
            },
        }    
    }
    async fn insert_tdb(&self, db_path: PathBuf) -> &Self{        
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
                        println!("[Tdb::<OPENING REPO METADATA FILE SUCCESSFUL>]");
                        match file.read_to_string(&mut contents).await {
                            Ok(_) => {
                                println!("[Tdb::<READING REPO METADATA FILE COMPLETE>]");
                                match ron::de::from_str::<Tdb>(&contents) {
                                    Ok(data) => {
                                        println!("[Tdb::<INITIALIZATION SUCCESSFUL>]");
                                        self.writer.lock().await.insert(file_name.clone().to_string_lossy().into(), Box::new(data));

                                        self
                                    },
                                    Err(error) => {
                                        eprintln!("[Tdb::<RON/SERDE DESERIALIZATION ERROR>]\n     {}", error);
                                        std::process::exit(1);
                                    }
                                }
                            },
                            Err(error) => {
                                eprintln!("[Tdb::<ERROR READING REPO METADATA>]\n     {}", error);
                                std::process::exit(1);
                            }
                        }
                    },
                    Err(error) => {
                        eprintln!("[Tdb::<ERROR OPENING FILE>]\n     {:#?}", error.kind());
                        std::process::exit(1);
                    }
                }
        }else {
            eprintln!("[Tdb::<ERROR GETTING DB NAME>]\n     `{:#?}`", db_path);
            std::process::exit(1);
        }
    }
    /*/// Create a new repository/directory that contains the databases
    pub async fn create(&self) -> Result<FileOps> {
        let mut repo_path = PathBuf::new();
        repo_path.push("TuringFeedsRepo");

        match DirBuilder::new().recursive(false).create(repo_path).await {
            Ok(_) => Ok(FileOps::CreateTrue),
            Err(error) => Err(turingfeeds_helpers::TuringFeedsError::IoError(error)),
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
                let data = ron::ser::to_string(&self.dbs)?;
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
    /// Deal with Document
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
    
    /// Create an empty document
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
                        .create_new(true)
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
    }
    

    pub async fn create_disk_field(path: &Path, key: &[u8], value: &[u8]) -> TransactionResult<DbOps, TuringFeedsError>{
        let db = Config::default()
            .create_new(true)
            .path(path)
            .open()?;

        if db.contains_key(key)? != true {

            db.transaction(|db| {
                db.insert(key, value)?;
                Ok(())
            })?;

            Ok(DbOps::FieldCreated)        
        }else {
            Ok(DbOps::FieldFound)
        }
    }

    pub async fn read_disk_field(path: &Path, key: &[u8]) -> TransactionResult<Option<IVec>> {

        if let Some(data) = sled::open(path)?.get(key)? {
            Ok(Some(data))
        }else {
            Ok(None)
        }
    }

    pub async fn update_disk_field(path: &Path, key: &[u8], value: &[u8]) -> TransactionResult<DbOps, TuringFeedsError>{
        let db = Config::default()
            .create_new(true)
            .path(path)
            .open()?;

        if db.contains_key(key)? == true {

            db.transaction(|db| {
                db.insert(key, value)?;
                Ok(())
            })?;

            Ok(DbOps::FieldCreated)        
        }else {
            Ok(DbOps::FieldNotFound)
        }
    }
    */
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
type DocumentsID = String;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Tdb {
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
    pub async fn memdb_new() -> Self {
        Self {
            datetime: TAI64N::now(),
            list: HashMap::new(),
        }
    }    
    /// Check whether the list are empty or not
    pub async fn memdb_is_empty(&self) -> bool {
        
        self.list.is_empty()
    }

    pub async fn memdb_insert(&mut self, identifier: &str, values: Documents) -> DbOps {
        if self.list.contains_key(identifier) == true {
            DbOps::DbAlreadyExists
        }else {
            self.list.insert(identifier.into(), values);

            DbOps::DbCreated
        }
    }

    pub async fn memdb_get(&self, identifier: &str) -> Option<&Documents> {
        self.list.get(identifier)
    }

    pub async fn memdb_update(&mut self, identifier: &str, values: Documents) -> DbOps {
        if self.list.contains_key(identifier) == true {
            self.list.insert(identifier.into(), values);

            DbOps::DbModified
        }else {
            DbOps::DbNotFound
        }
    }

    pub async fn memdb_remove(&mut self, identifier: &str) -> DbOps {
        if let Some(_) = self.list.remove(identifier) {
            DbOps::DbDropped
        }else {
            DbOps::DbNotFound
        }
    }

    pub async fn memdb_swap(&mut self, values: &Tdb) -> &Self {
        self.datetime = values.datetime.clone();
        self.list = values.list.clone();

        self
    }
    /// Create a new database that contains the databases
    pub async fn db_create(&self, identifier: &str) -> Result<FileOps> {
        let mut db_path = PathBuf::new();
        db_path.push("TuringFeedsRepo");
        db_path.push(identifier);
        

        match DirBuilder::new().recursive(false).create(db_path).await {
            Ok(_) => Ok(FileOps::CreateTrue),
            Err(error) => Err(turingfeeds_helpers::TuringFeedsError::IoError(error)),
        }
    }
    /// Create a new repository/directory that contains the databases
    pub async fn db_drop(&self, identifier: &str) -> Result<FileOps> {
        let mut db_path = PathBuf::new();
        db_path.push("TuringFeedsRepo");
        db_path.push(identifier);

        match fs::remove_dir(db_path).await {
            Ok(_) => Ok(FileOps::DeleteTrue),
            Err(error) => Err(TuringFeedsError::IoError(error)),
        }
    }
    /// Create the Metadata file or add data to the metadata file
    pub async fn db_commit(&self, identifier: &str) -> Result<FileOps> {
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
}

type SledDocumentName = String;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Documents {
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
    pub async fn new() -> Self {
        Self { list: HashMap::new() }
    }    
    /// Check whether the list are empty or not
    pub async fn is_empty(&self) -> bool {
        
        self.list.is_empty()
    }

    pub async fn insert(&mut self, identifier: &str, values: Fields) -> DbOps {
        if self.list.contains_key(identifier) == true {
            DbOps::DocumentAlreadyExists
        }else {
            self.list.insert(identifier.into(), values);

            DbOps::DocumentInserted
        }
    }

    pub async fn get(&self, identifier: &str) -> Option<&Fields> {
        self.list.get(identifier)
    }

    pub async fn update(&mut self, identifier: &str, values: &Fields) -> DbOps {
        if self.list.contains_key(identifier) == true {
            self.list.insert(identifier.into(), values.clone());

            DbOps::DocumentModified
        }else {
            DbOps::DocumentNotFound
        }
    }

    pub async fn remove(&mut self, identifier: &str) -> DbOps {
        if let Some(_) = self.list.remove(identifier) {
            DbOps::DocumentDropped
        }else {
            DbOps::DocumentNotFound
        }
    }

    pub async fn swap(&mut self, value: &Documents) -> &Self {
        self.list = value.list.clone();
        

        self
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Fields {
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
    pub async fn new() -> Self {

        Self {
            list: HashMap::default(),
        }
    }
    /// Check whether the fields are empty or not
    pub async fn is_empty(&self) -> bool {
        
        self.list.is_empty()
    }

    pub async fn insert(&mut self, identifier: &str, value: FieldMetadata) -> DbOps {
        
        if self.list.contains_key(identifier) == true { 

            DbOps::FieldAlreadyExists
        }else {
            self.list.insert(identifier.into(), value);

            drop(identifier);
            
            DbOps::FieldInserted
        }
    }
    /// Get the field returning (FieldName, FieldMetadata)
    pub async fn get(&self, identifier: &str) -> Option<&FieldMetadata> {

        match self.list.get(identifier) {
            Some(field) => Some(field),
            None => None,
        }
    }
    /// Get only the key since the value only updates the time modified 
    /// which is done automatically by the `FieldMetadata::new().await.update_modified_time().await` method
    pub async fn update(&mut self, identifier: &str) -> DbOps {

        match self.list.get_mut(identifier) {
            Some(field) => {
                field.update_modified_time().await;
                
                DbOps::FieldModified
            }
            None => DbOps::FieldNotFound,
        }
    }

    pub async fn remove(&mut self, identifier: &str) -> DbOps {

        if let Some(_) = self.list.remove(identifier) {
            DbOps::FieldDropped
        }else {
            DbOps::FieldNotFound
        }
    }

    pub async fn swap(&mut self, value: &Fields) -> &Self {
        self.list = value.list.clone();

        self
    }
    
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
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

    pub async fn swap(&mut self, value: &FieldMetadata) -> &Self {
        self.create_time = value.create_time;
        self.modified_time = value.modified_time;

        self
    }

    pub async fn update_modified_time(&mut self) -> &Self {
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

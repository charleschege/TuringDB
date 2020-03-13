use async_std::{
    fs,
    fs::{DirBuilder, OpenOptions},
    io::prelude::*,
    path::PathBuf,
    sync::{Arc, Mutex},
    stream::StreamExt,
};
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

#[allow(non_upper_case_globals)]
static TuringFeedsRepo: &'static str = "TuringFeedsRepo";

lazy_static!{
    pub static ref REPO: TuringFeeds = {

        TuringFeeds::new()
    };

    pub static ref READER: Arc<Mutex<ReadHandle<String, Box<Tdb>>>> = REPO.reader.clone();
    pub static ref WRITER:Arc<Mutex<WriteHandle<String, Box<Tdb>>>> = REPO.writer.clone();
}

/// Handle list of databases
#[derive(Debug)]
pub struct TuringFeeds {
    reader: Arc<Mutex<ReadHandle<String, Box<Tdb>>>>,
    writer: Arc<Mutex<WriteHandle<String, Box<Tdb>>>>,
}

impl TuringFeeds {
    /// Initialize the structure with default values
    pub fn new() -> Self {
        let (reader, writer) = evmap::new::<String, Box<Tdb>>();
        Self {
            reader: Arc::new(Mutex::new(reader)),
            writer: Arc::new(Mutex::new(writer)),
        }
    }     
    /// Check whether the list are empty or not
    pub async fn repo_is_empty(&self) -> bool {
        
        self.reader.lock().await.is_empty()
    }
    /// Recursively walk through the Directory
    /// Load all the Directories into memory
    /// Hash and Compare with Persisted Hash to check for corruption
    /// Throw errors if any otherwise    
    pub async fn repo_init(&self) -> &Self{
        let mut repo_path = PathBuf::new();
        repo_path.push(TuringFeedsRepo);
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
                self
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
    pub async fn repo_create(&self) -> Result<DbOps> {
        let mut repo_path = PathBuf::new();
        repo_path.push(TuringFeedsRepo);

        DirBuilder::new().recursive(false).create(repo_path).await?;
        self.create_ops_log_file().await?;
        self.create_errors_log_file().await?;
        Ok(DbOps::RepoCreated)
    }
    /// Create a new repository/directory that contains the databases
    pub async fn repo_drop(&self) -> Result<DbOps> {
        // TODO - list all the databases and their fields that are being dropped and log to `ops.log` file
        let mut repo_path = PathBuf::new();
        repo_path.push(TuringFeedsRepo);

        fs::remove_dir_all(repo_path).await?;

        Ok(DbOps::RepoDropped)
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
        if self.reader.lock().await.contains_key(db_name) == true {
            Ok(DbOps::DbAlreadyExists)
        }else {
            let db = Tdb::new().await;
            self.create_on_disk(db_name, &db).await?;
            
            self.writer.lock().await.insert(db_name.into(), Box::new(db));
            self.writer.lock().await.refresh();
            Ok(DbOps::DbCreated)
        }
    }
    /// Remove a Database if it exists
    pub async fn db_drop(&self, db_name: &str) -> Result<DbOps> {
        if self.repo_is_empty().await == true {
            Ok(DbOps::RepoEmpty)
        }else {
            if let Some(_) = self.db_get(db_name).await {
                self.disk_db_drop(db_name).await?;
                self.writer.lock().await.empty(db_name.into());
                self.writer.lock().await.refresh();
                Ok(DbOps::DbDropped)
            }else {
                Ok(DbOps::DbNotFound)
            }
        }
    } 
    /// Get list of databases
    pub async fn db_list(&self) -> DbOps {
        if self.repo_is_empty().await == true {
            DbOps::RepoEmpty
        }else {
            let mut db_list: Vec<String> = Vec::new();
            for (key, _) in &self.reader.lock().await.read() {
                db_list.push(key.into());
            }

            if db_list.is_empty() == true {
                DbOps::RepoEmpty
            }else {
                DbOps::DbList(db_list)
            }
        }
    }
    /// Get the documents of a database
    pub async fn db_read(&self, db_name: &str) -> DbOps {
        if self.repo_is_empty().await == true {
            DbOps::RepoEmpty
        }else {
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
        
    }
    /// Create a new repository/directory that contains the databases
    async fn create_ops_log_file(&self) -> Result<FileOps> {
        let mut log_file_path = PathBuf::new();
        log_file_path.push(TuringFeedsRepo);
        log_file_path.push("ops.log");

        OpenOptions::new()
            .create(true)
            .write(true)
            .open(log_file_path)
            .await?;
        
        Ok(FileOps::CreateTrue)
    }
    /// Create a new repository/directory that contains the databases
    async fn create_errors_log_file(&self) -> Result<FileOps> {
        let mut log_file_path = PathBuf::new();
        log_file_path.push(TuringFeedsRepo);
        log_file_path.push("errors.log");

        OpenOptions::new()
            .create(true)
            .write(true)
            .open(log_file_path)
            .await?;
        
        Ok(FileOps::CreateTrue)
    }
    /// Get a Database if it exists. This function is not public and is used by methods in this `impl TuringFeeds` block
    /// to get the documents in a given database
    async fn db_get(&self, db_name: &str) -> Option<Tdb> {
        match self.reader.lock().await.get(db_name) {
            Some(value) => { 
                if let Some(value) = value.iter().next().clone() {
                    Some(*value.clone())//
                }else {//
                    None
                }
            },
            None => { None }
        }
    }
    /// Create a new database that contains the databases
    async fn create_on_disk(&self, db_name: &str, db: &Tdb) -> Result<FileOps> {
        let mut db_path = PathBuf::new();
        db_path.push(TuringFeedsRepo);
        db_path.push(db_name);
        DirBuilder::new().recursive(false).create(db_path).await?;

        self.commit(db_name, db).await
    }
    /// Create the Metadata file or add data to the metadata file
    async fn commit(&self, db_name: &str, db: &Tdb) -> Result<FileOps> {
        let mut db_path = PathBuf::new();
        db_path.push(TuringFeedsRepo);
        db_path.push(db_name);
        db_path.push(db_name);
        db_path.set_extension("log");

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(db_path)
            .await?;
        let data = ron::ser::to_string(db)?;
        file.write_all(&data.into_bytes()).await?;
        file.sync_all().await?;
        //writeln!(&file, "{}", data).await?;

        Ok(FileOps::WriteTrue)
    }
    /// Create a new repository/directory that contains the databases
    async fn disk_db_drop(&self, identifier: &str) -> Result<FileOps> {
        let mut db_path = PathBuf::new();
        db_path.push(TuringFeedsRepo);
        db_path.push(identifier);
        fs::remove_dir_all(db_path).await?;
        
        Ok(FileOps::DeleteTrue)
    }
    
    ///Handle A Document
    /// 
    /// Create a Document backed by Sled database
    pub async fn document_create(&self, values: DocumentOnly) -> Result<DbOps> {
        if self.repo_is_empty().await == true {
            Ok(DbOps::RepoEmpty)
        }else {
            let mut document_path = PathBuf::new();
            document_path.push(TuringFeedsRepo);
            document_path.push(&values.db);
            document_path.push(&values.document);

            if let Some(mut db) = self.db_get(&values.db).await {
                sled::Config::default()
                    .create_new(true)
                    .path(document_path)
                    .open()?;
                
                db.list.insert(values.document, Document::new().await);

                self.commit(&values.db, &db).await?;
                self.writer.lock().await.update(values.db, Box::new(db));
                self.writer.lock().await.refresh();

                Ok(DbOps::DocumentCreated)
            }else {
                Ok(DbOps::DbNotFound)
            }
        }
    }
    /// Read document fields returning a DbOps::FieldList(String)
    pub async fn document_read(&self, values: DocumentOnly) -> DbOps {
        if self.repo_is_empty().await == true {
            DbOps::RepoEmpty
        }else {
            if let Some(db) = self.db_get(&values.db).await {
                if let Some(document) = db.list.get(&values.document) {
                    let mut fields: Vec<String> = Vec::new();
                    for value in document.list.keys() {
                        fields.push(value.into());
                    }

                if fields.is_empty() == true {
                    DbOps::DocumentEmpty
                }else {
                        DbOps::FieldList(fields)
                }
                }else {
                    DbOps::DocumentNotFound
                }
            }else {
                DbOps::DbNotFound
            }
        }
    }
    /// Drop a document
    pub async fn document_drop(&self, values: DocumentOnly) -> Result<DbOps> {
        if self.repo_is_empty().await == true {
            Ok(DbOps::RepoEmpty)
        }else {
            if let Some(mut db) = self.db_get(&values.db).await {
                if let Some(_) = db.list.get(&values.document) {
                    let mut document_path = PathBuf::new();
                    document_path.push(TuringFeedsRepo);
                    document_path.push(&values.db);
                    document_path.push(&values.document);
    
                    fs::remove_dir_all(document_path).await?;
                    db.list.remove(&values.document);
                    self.commit(&values.db, &db).await?;
                    self.writer.lock().await.update(values.db, Box::new(db));
                    self.writer.lock().await.refresh();
                    
                    Ok(DbOps::DocumentDropped)
                }else {
                    Ok(DbOps::DocumentNotFound)
                }
            }else {
                Ok(DbOps::DbNotFound)
            }
        }
    }

    /// Handle A Field
    /// 
    /// CRUDL
    /// Insert a field into a specified document
    pub async fn field_insert(&self, values: FieldWithData) -> Result<DbOps> {
        if self.repo_is_empty().await == true {
            Ok(DbOps::RepoEmpty)
        }else {
            if let Some(mut db) = self.db_get(&values.db).await {
                if let Some(document) = db.list.get_mut(&values.document) {
                    let mut document_path = PathBuf::new();
                    document_path.push(TuringFeedsRepo);
                    document_path.push(&values.db);
                    document_path.push(&values.document);

                    let sled_document = sled::Config::default()
                        .create_new(false)
                        .path(document_path)
                        .open()?;

                    if sled_document.contains_key(&values.field.as_bytes())? == true {
                        Ok(DbOps::FieldAlreadyExists)
                    }else{
                        sled_document.insert(values.field.clone().into_bytes(), values.data)?;

                        document.list.insert(values.field, FieldMetadata::new().await);
                        self.commit(&values.db, &db).await?;
                        self.writer.lock().await.update(values.db, Box::new(db));
                        self.writer.lock().await.refresh();                    

                        Ok(DbOps::FieldInserted)
                    }
                }else {
                    Ok(DbOps::DocumentNotFound)
                }
            }else {
                Ok(DbOps::DbNotFound)
            }
        }
    }
    /// Read the contents of a field
    pub async fn field_get(&self, values: FieldNoData) -> Result<DbOps> {
        if self.repo_is_empty().await == true {
            Ok(DbOps::RepoEmpty)
        }else {
            let db_name = values.db;
            let document_name = values.document;
            let field_name = values.field;

            if let Some(db) = self.db_get(&db_name).await {
                if let Some(_) = db.list.get(&document_name) {
                    let mut document_path = PathBuf::new();
                    document_path.push(TuringFeedsRepo);
                    document_path.push(&db_name);
                    document_path.push(&document_name);

                    let sled_document = sled::Config::default()
                        .create_new(false)
                        .path(document_path)
                        .open()?;

                    if let Some(field) = sled_document.get(&field_name.as_bytes())? {
                        Ok(DbOps::FieldContents(field.to_vec()))
                    }else {
                        Ok(DbOps::FieldNotFound)
                    }
                }else {
                    Ok(DbOps::DocumentNotFound)
                }
            }else {
                Ok(DbOps::DbNotFound)
            }
        }
    }
    /// Remove a field from a specific document
    pub async fn field_drop(&self, values: FieldNoData) -> Result<DbOps> {
        if self.repo_is_empty().await == true {
            Ok(DbOps::RepoEmpty)
        }else {
            if let Some(mut db) = self.db_get(&values.db).await {
                if let Some(document) = db.list.get_mut(&values.document) {
                    let mut document_path = PathBuf::new();
                    document_path.push(TuringFeedsRepo);
                    document_path.push(&values.db);
                    document_path.push(&values.document);

                    let sled_document = sled::Config::default()
                        .create_new(false)
                        .path(document_path)
                        .open()?;

                    if let Some(_) = sled_document.transaction::<_,_, sled::Error>(|doc| {
                            Ok(doc.remove(values.field.clone().into_bytes())?)
                        })?
                        {

                            document.list.remove(&values.field);
                            self.commit(&values.db, &db).await?;
                            self.writer.lock().await.update(values.db, Box::new(db));
                            self.writer.lock().await.refresh();                    

                            Ok(DbOps::FieldDropped)
                    }else {
                        Ok(DbOps::FieldNotFound)
                    }
                }else {
                    Ok(DbOps::DocumentNotFound)
                }
            }else {
                Ok(DbOps::DbNotFound)
            }
        }
    }
    /// Update the contents of a field in a specific document
    pub async fn field_update(&self, values: FieldWithData) -> Result<DbOps> {
        if self.repo_is_empty().await == true {
            Ok(DbOps::RepoEmpty)
        }else {
            if let Some(mut db) = self.db_get(&values.db).await {
                if let Some(document) = db.list.get_mut(&values.document) {
                    let mut document_path = PathBuf::new();
                    document_path.push(TuringFeedsRepo);
                    document_path.push(&values.db);
                    document_path.push(&values.document);

                    if let Some(field) = document.list.get_mut(&values.field) {
                        let sled_document = sled::Config::default()
                        .create_new(false)
                        .path(document_path)
                        .open()?;

                        if sled_document.contains_key(&values.field.as_bytes())? == true {
                            sled_document.insert(values.field.clone().into_bytes(), values.data)?;
                            field.modified_time = TAI64N::now();

                            self.commit(&values.db, &db).await?;
                            self.writer.lock().await.update(values.db, Box::new(db));
                            self.writer.lock().await.refresh();

                            Ok(DbOps::FieldModified)
                        }else{
                            Ok(DbOps::FieldNotFound)
                        }
                    }else{
                        Ok(DbOps::FieldNotFound)
                    }
                }else {
                    Ok(DbOps::DocumentNotFound)
                }
            }else {
                Ok(DbOps::DbNotFound)
            }
        }
    }
}

#[derive(Debug)]
struct LogFile<T> {
    error: Result<T>,
    timestamp: TAI64N,
}

impl<T> LogFile<T> where T: std::fmt::Debug {
    ///Write to logger file
    async fn log_error_to_file(&self, error: T) -> Result<()> {
        let mut log_file_path = PathBuf::new();
        log_file_path.push(TuringFeedsRepo);
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

// TODO CRUD
// TODO Sled CRUD

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

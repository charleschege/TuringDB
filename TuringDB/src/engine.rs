use anyhow::Result;
use async_lock::Lock;
use custom_codes::DbOps;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    ffi::OsString,
    fs::{self, DirBuilder, OpenOptions},
    path::PathBuf,
};
use tai64::TAI64N;
use blocking::unblock;

const REPO_NAME: &'static str = "TuringDB_Repo";

#[derive(Debug, Clone)]
pub struct TuringEngine {
    dbs: DashMap<OsString, Tdb>, // Repo<DatabaseName, Databases>
}

impl TuringEngine {
    /// Create a new in-memory repo
    pub async fn new() -> TuringEngine {
        Self {
            dbs: DashMap::new(),
        }
    }
    /// Create a repo
    pub async fn repo_create(&self) -> Result<DbOps> {
        let path = "TuringDB_Repo";
        unblock!(DirBuilder::new().recursive(false).create(path))?;

        self.create_ops_log_file().await?;
        self.create_errors_log_file().await?;

        Ok(DbOps::RepoCreated)
    }
    /// Create a new repository/directory that contains the databases
    async fn create_ops_log_file(&self) -> Result<()> {
        unblock!(OpenOptions::new()
            .create_new(true)
            .write(true)
            .open("TuringDB_Repo/ops.log"))?;
        Ok(())
    }
    /// Create a new repository/directory that contains the databases
    async fn create_errors_log_file(&self) -> Result<()> {
        unblock!(OpenOptions::new()
            .create_new(true)
            .write(true)
            .open("TuringDB_Repo/errors.log"))?;

        Ok(())
    }
    /// Check if the repository is empty
    pub async fn is_empty(&self) -> bool {
        self.dbs.is_empty()
    }
    ///TODO
    /// 1. READ THE REPO AND CHECK AGANIST A HMAC FOR TIME AND HASHES
    /// 5. APPLY TIMESTAMP AND DATABASE OPS TO ops.log file
    ///---------
    /// Read a repo
    pub async fn repo_init(&self) -> Result<&TuringEngine> {
        let mut repo = unblock!(fs::read_dir("TuringDB_Repo"))?;
        while let Some(database_entry) = repo.next() {
            let database_entry = database_entry?;
            let database_name = database_entry.file_name();

            if database_entry.file_type()?.is_dir() == true {
                let mut repo = unblock!(fs::read_dir(&database_entry.path()))?;
                let mut current_db = Tdb::new().await;

                while let Some(document_entry) = repo.next() {
                    let document_entry = document_entry?;
                    let mut field_keys = Vec::new();

                    dbg!(&document_entry);

                    if document_entry.file_type()?.is_dir() == true {
                        let document_name = document_entry.file_name();
                        let db = sled::open(document_entry.path())?;

                        for field_key in db.into_iter().keys() {
                            field_keys.push(String::from_utf8(field_key?.to_vec())?);
                        }

                        current_db.list.insert(document_name, Document {
                            fd: Lock::new(db),
                            keys: field_keys,
                        });
                    }

                    dbg!(&current_db);
                }
                self.dbs.insert(database_name, current_db);
            }
        }

        Ok(self)
    }
    /// Drop a repository
    pub async fn repo_drop(&self) -> Result<DbOps> {
        unblock!(fs::remove_dir_all(REPO_NAME))?;
        Ok(DbOps::RepoDropped)
    }

    /************** DATABASES *******************/
    /// Create a database
    pub async fn db_create(&self, db_name: &str) -> Result<DbOps> {
        let mut path: PathBuf = REPO_NAME.into();
        path.push(db_name);

        unblock!(DirBuilder::new().recursive(false).create(path))?;

        self.dbs.insert(db_name.into(), Tdb::new().await);

        Ok(DbOps::DbCreated)
    }
    /// Drop the database
    pub async fn db_drop(&self, db_name: &str) -> Result<DbOps> {
        let mut path: PathBuf = REPO_NAME.into();
        path.push(db_name);
        unblock!(fs::remove_dir_all(path))?;

        self.dbs.remove(&OsString::from(db_name));

        Ok(DbOps::DbDropped)
    }
    /// List all the databases in the repo
    pub async fn db_list(&self) -> DbOps {
        let mut list: Vec<String> = Vec::new();

        for db in self.dbs.iter() {
            list.push(db.key().clone().to_string_lossy().to_string());
        }

        DbOps::DbList(list)
    }

    /************** DOCUMENTS ************/
    /// Create a document
    pub async fn doc_create(&self, db_name: &str, doc_name: &str) -> Result<DbOps> {
        if let Some(mut database) = self.dbs.get_mut(&OsString::from(db_name)) {
            let mut path: PathBuf = REPO_NAME.into();
            path.push(db_name);
            path.push(doc_name);            

            database
                .value_mut()
                .list
                .insert(OsString::from(doc_name), Document {
                    fd: Lock::new(sled::Config::default().create_new(true).path(path).open()?),
                    keys: Vec::new(),
                });

            Ok(DbOps::DocumentCreated)
        } else {
            Ok(DbOps::DbNotFound)
        }
    }
    /// Drop a document
    pub async fn doc_drop(&self, db_name: &str, doc_name: &str) -> Result<DbOps> {
        if let Some(mut database) = self.dbs.get_mut(&OsString::from(db_name)) {
            match database.value_mut().list.remove(&OsString::from(doc_name)) {
                Some(_) => {
                    let mut path: PathBuf = REPO_NAME.into();
                    path.push(db_name);
                    path.push(doc_name);
                    unblock!(fs::remove_dir_all(path))?;

                    Ok(DbOps::DocumentDropped)
                }
                None => Ok(DbOps::DocumentNotFound),
            }
        } else {
            Ok(DbOps::DbNotFound)
        }
    }
    /// List all fields in a document
    pub async fn doc_list(&self, db_name: &str) -> DbOps {
        if let Some(database) = self.dbs.get(&OsString::from(db_name)) {
            let list = database
                .list
                .keys()
                .into_iter()
                .map(|document| document.to_string_lossy().to_string())
                .collect::<Vec<String>>();

            DbOps::DocumentList(list)
        } else {
            DbOps::DbNotFound
        }
    }

    /************* FIELDS ************/
    /// List all fields in a document
    pub async fn field_list(&self, db_name: &str, doc_name: &str) -> DbOps {
        if let Some(mut database) = self.dbs.get_mut(&OsString::from(db_name)) {
            if let Some(document) = database.value_mut().list.get_mut(&OsString::from(doc_name)) {
                DbOps::FieldList(document.keys.to_owned())
            } else {
                DbOps::DocumentNotFound
            }
        } else {
            DbOps::DbNotFound
        }
    }
    /// Create a field with data
    pub async fn field_create(
        &self,
        db_name: &str,
        doc_name: &str,
        field_name: &str,
        data: &[u8],
    ) -> Result<DbOps> {
        if let Some(mut database) = self.dbs.get_mut(&OsString::from(db_name)) {
            if let Some(document) = database.value_mut().list.get_mut(&OsString::from(doc_name)) {
                if let Ok(_) = document.keys.binary_search(&field_name.into()) {
                    Ok(DbOps::FieldAlreadyExists)
                } else {
                    let field_data = FieldData::new(data).await;
                    let field_data = bincode::serialize::<FieldData>(&field_data)?;
                    let field_key: Vec<u8> = field_name.to_owned().into_bytes();

                    document.fd.lock().await.transaction::<_, _, sled::Error>(|db| {
                        Ok(db.insert(field_key.clone(), field_data.clone().to_vec())?)
                    })?;

                    document.keys.push(field_name.into());

                    Ok(DbOps::FieldInserted)
                }
            } else {
                Ok(DbOps::DocumentNotFound)
            }
        } else {
            Ok(DbOps::DbNotFound)
        }
    }
    /// Get a field
    pub async fn field_get(
        &self,
        db_name: &str,
        doc_name: &str,
        field_name: &str,
    ) -> Result<DbOps> {
        if let Some(mut database) = self.dbs.get_mut(&OsString::from(db_name)) {
            if let Some(document) = database.value_mut().list.get_mut(&OsString::from(doc_name)) {
                if let Ok(_) = document.keys.binary_search(&field_name.into()) {
                    let field_key: Vec<u8> = field_name.to_owned().into_bytes();

                    match document.fd.lock().await.get(field_key)? {
                        Some(data) => Ok(DbOps::FieldContents(data.to_vec())),
                        None => Ok(DbOps::FieldNotFound),
                    }
                } else {
                    Ok(DbOps::FieldNotFound)
                }
            } else {
                Ok(DbOps::DocumentNotFound)
            }
        } else {
            Ok(DbOps::DbNotFound)
        }
    }
    /// Drop a field
    pub async fn field_drop(
        &self,
        db_name: &str,
        doc_name: &str,
        field_name: &str,
    ) -> Result<DbOps> {
        if let Some(mut database) = self.dbs.get_mut(&OsString::from(db_name)) {
            if let Some(document) = database.value_mut().list.get_mut(&OsString::from(doc_name)) {
                if let Ok(field_index) = document.keys.binary_search(&field_name.into()) {
                    let field_key: Vec<u8> = field_name.to_owned().into_bytes();

                    match document.fd.lock().await.remove(field_key)? {
                        Some(_) => {
                            document.keys.remove(field_index);
                            Ok(DbOps::FieldDropped)
                        }
                        None => Ok(DbOps::FieldNotFound),
                    }
                } else {
                    Ok(DbOps::FieldNotFound)
                }
            } else {
                Ok(DbOps::DocumentNotFound)
            }
        } else {
            Ok(DbOps::DbNotFound)
        }
    }
    /// Update a field
    pub async fn field_update(
        &self,
        db_name: &str,
        doc_name: &str,
        field_name: &str,
        field_value: &[u8],
    ) -> Result<DbOps> {
        if let Some(mut database) = self.dbs.get_mut(&OsString::from(db_name)) {
            if let Some(document) = database.value_mut().list.get_mut(&OsString::from(doc_name)) {
                if let Ok(_) = document.keys.binary_search(&field_name.into()) {
                    let field_key: Vec<u8> = field_name.to_owned().into_bytes();
                    let field_key_insert = field_key.clone();
                    let stored_data;

                    let key_exists = document.fd.lock().await.get(field_key_insert)?;
                    
                    match key_exists {
                        Some(data) => {
                            stored_data = data.to_vec();
                            let mut current_field_data = bincode::deserialize::<FieldData>(&stored_data)?;
                            current_field_data.update(field_value).await;
                            let modified_field_data = bincode::serialize(&current_field_data)?;
                            match document.fd.lock().await.insert(field_key, modified_field_data)? {
                                Some(_) => Ok(DbOps::FieldModified),
                                // FIXME Decide what to do in case the field didnt exist
                                // Maybe push these to the database logs and alert DB Admin
                                None => Ok(DbOps::FieldInserted),
                            }
                        }
                        None => Ok(DbOps::FieldNotFound),
                    }
                } else {
                    Ok(DbOps::FieldNotFound)
                }
            } else {
                Ok(DbOps::DocumentNotFound)
            }
        } else {
            Ok(DbOps::DbNotFound)
        }
    }
}

#[derive(Debug, Clone)]
struct Tdb {
    list: HashMap<OsString, Document>, 
    //Database<Document, Fileds>
    //rights: Option<HashMap<UserIdentifier, (Role, AccessRights)>>,
    //database_hash: Blake2hash,
    //secrecy: TuringSecrecy,
    //config: TuringConfig,
    //authstate: Assymetric Crypto
    //superuser: Only one
    // admins: vec![], -> (User, PriveledgeAccess)
    //users: vec![] -> """"
}

impl Tdb {
    async fn new() -> Tdb {
        Self {
            list: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct Document {
    fd: Lock<sled::Db>,
    keys: Vec<String>
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct FieldData {
    data: Vec<u8>,
    created: TAI64N,
    modified: TAI64N,
}

impl FieldData {
    pub async fn new(value: &[u8]) -> FieldData {
        let current_time = TAI64N::now();

        Self {
            data: value.into(),
            created: current_time,
            modified: current_time,
        }
    }

    pub async fn update(&mut self, value: &[u8]) -> &FieldData {
        self.data = value.into();
        self.modified = TAI64N::now();

        self
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

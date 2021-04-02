use crate::{Document, OpsOutcome, RepoPath, TuringDB, TuringDbError};
use anyhow::Result;
use async_fs::{self, DirBuilder, ReadDir};
use async_lock::Mutex;
use camino::{Utf8Path, Utf8PathBuf};
use dashmap::DashMap;
use futures_lite::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ffi::OsString, io::ErrorKind};
use tai64::TAI64N;

// TODO use custom_codes errors to give actual errors
// TODO Check whether you can respond with sled::Error
// TODO move repo files to home user

/// This engine handles data all database queries and in-memory keys and sled file locks
/// #### Structure
/// ```
/// #[derive(Debug, Clone)]
/// pub struct TuringEngine {
///     dbs: DashMap<Utf8Path, Tdb>, // Repo<DatabaseName, Databases>
/// }
/// ```
#[derive(Debug)]
pub struct TuringEngine {
    dbs: DashMap<Utf8PathBuf, TuringDB>, // Repo<DatabaseName, Databases>
    repo_dir: Utf8PathBuf,
}
impl TuringEngine {
    /// Create a new in-memory repo
    pub async fn new() -> Result<TuringEngine, TuringDbError> {
        let path = RepoPath::access_dir().await?;

        Ok(Self {
            dbs: DashMap::new(),
            repo_dir: path,
        })
    }

    pub async fn get_repo_dir(&self) -> &Utf8PathBuf {
        &self.repo_dir
    }

    /// Create a repo
    pub async fn repo_create(&self) -> Result<OpsOutcome, TuringDbError> {
        DirBuilder::new()
            .recursive(false)
            .create(&self.repo_dir)
            .await?;

        Ok(OpsOutcome::RepoCreated)
    }
    /// Check if the repository is empty
    pub fn is_empty(&self) -> bool {
        self.dbs.is_empty()
    }
    pub async fn repo_init(&mut self) -> Result<OpsOutcome, TuringDbError> {
        let mut repo = async_fs::read_dir(&self.repo_dir).await?;

        while let Some(database_entry) = repo.try_next().await? {
            let database_name_raw = database_entry.file_name();

            if database_entry.file_type().await?.is_dir() {
                let mut repo = async_fs::read_dir(&database_entry.path()).await?;
                let mut current_db = TuringDB::new();

                while let Some(document_entry) = repo.try_next().await? {
                    let mut field_keys = Vec::new();

                    if document_entry.file_type().await?.is_dir() {
                        let document_name_raw = document_entry.file_name();
                        let document_name: Utf8PathBuf =
                            TuringEngine::to_utf8_path(document_name_raw)?;

                        let db = sled::open(document_entry.path())?;

                        for field_key in db.into_iter().keys() {
                            field_keys.push(field_key?);
                        }

                        let data = field_keys.iter().map(|inner| inner.to_vec()).collect();

                        current_db.list.insert(
                            document_name.into(),
                            Document {
                                fd: Mutex::new(db),
                                keys: data,
                            },
                        );
                    }
                }

                let database_name: Utf8PathBuf = TuringEngine::to_utf8_path(database_name_raw)?;
                self.dbs
                    .insert(Utf8PathBuf::from(database_name), current_db);
            }
        }

        Ok(OpsOutcome::RepoInitialized)
    }

    pub async fn db_create(&self, db_name: &str) -> Result<OpsOutcome, TuringDbError> {
        let db_path = Utf8Path::new(db_name).to_path_buf();
        let db = TuringDB::new();

        let dbop = db.db_create(&self.repo_dir, &db_path).await?;

        self.dbs.insert(db_path.into(), TuringDB::new());
        Ok(dbop)
    }

    pub async fn db_drop(&self, db_name: &str) -> Result<OpsOutcome, TuringDbError> {
        let db_path = Utf8Path::new(db_name).to_path_buf();
        let db = TuringDB::new();

        let dbop = db.db_drop(&self.repo_dir, &db_path).await?;

        match self.dbs.remove(&db_path) {
            Some(_) => Ok(dbop),
            None => Err(TuringDbError::NotFound),
        }
    }
    /// List all the databases in the repo
    pub fn db_list(&self) -> OpsOutcome {
        let list = self
            .dbs
            .iter()
            .map(|db| db.key().into())
            .collect::<Vec<Utf8PathBuf>>();

        if list.is_empty() {
            OpsOutcome::RepoEmpty
        } else {
            OpsOutcome::DbList(list)
        }
    }
    /// List all the databases in the repo
    pub fn db_list_sorted(&self) -> OpsOutcome {
        let mut list = self
            .dbs
            .iter()
            .map(|db| db.key().into())
            .collect::<Vec<Utf8PathBuf>>();

        list.sort();

        if list.is_empty() {
            OpsOutcome::RepoEmpty
        } else {
            OpsOutcome::DbList(list)
        }
    }
    /// List all the documents in the database in any order
    pub fn document_list(&self, db_name: &str) -> OpsOutcome {
        let db_name = Utf8Path::new(db_name);
        match self.dbs.get(&db_name.to_path_buf()) {
            None => OpsOutcome::DbNotFound,
            Some(db) => TuringDB::document_list(&db),
        }
    }
    /// List all documents in a database sorted alphabetically
    pub fn document_list_sorted(&self, db_name: &str) -> OpsOutcome {
        let db_name = Utf8Path::new(db_name);
        match self.dbs.get(&db_name.to_path_buf()) {
            None => OpsOutcome::DbNotFound,
            Some(db) => TuringDB::document_list_sorted(&db),
        }
    }

    fn to_utf8_path(value: OsString) -> Result<Utf8PathBuf, TuringDbError> {
        match std::path::PathBuf::from(value).to_str() {
            None => Err(TuringDbError::PathReadIsNotUtf8Path),
            Some(path) => Ok(Utf8Path::new(path).to_path_buf()),
        }
    }
}

/*//TODO
// 1. READ THE REPO AND CHECK AGANIST A HMAC FOR TIME AND HASHES
// 5. APPLY TIMESTAMP AND DATABASE OPS TO ops.log file
//---------
/// Read a repo

pub async fn repo_init(&mut self) -> Result<OpsOutcome, TuringDbError> {
    let mut repo = match async_fs::read_dir(&self.repo_dir).await {
        Ok(value) => value,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                return Ok(self);
            } else {
                return Err(anyhow::Error::new(e));
            }
        }
    };

    while let Some(database_entry) = repo.try_next().await? {
        let database_name = database_entry.file_name();

        if database_entry.file_type().await?.is_dir() {
            let mut repo = async_fs::read_dir(&database_entry.path()).await?;
            let mut current_db = Tdb::new();

            while let Some(document_entry) = repo.try_next().await? {
                let mut field_keys = Vec::new();

                if document_entry.file_type().await?.is_dir() {
                    let document_name = document_entry.file_name();
                    let db = sled::open(document_entry.path())?;

                    for field_key in db.into_iter().keys() {
                        field_keys.push(field_key?);
                    }

                    let data = field_keys.iter().map(|inner| inner.to_vec()).collect();

                    current_db.list.insert(
                        document_name,
                        Document {
                            fd: Mutex::new(db),
                            keys: data,
                        },
                    );
                }
            }
            self.dbs.insert(database_name, current_db);
        }
    }

    Ok(self)
}*/
/*
/// Drop a repository
pub async fn repo_drop(&self) -> Result<DbOps> {
    async_fs::remove_dir_all(REPO_NAME).await?;
    Ok(DbOps::RepoDropped)
}*/

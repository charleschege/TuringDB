use crate::{Document, OpsOutcome, RepoUtf8Path, TuringDB, TuringDbError};
use anyhow::Result;
use async_fs::{self, DirBuilder, ReadDir};
use async_lock::Mutex;
use camino::{Utf8Path, Utf8PathBuf};
use dashmap::DashMap;
use futures_lite::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    ffi::OsString,
    io::ErrorKind,
    path::{Path, PathBuf},
};
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
        let path = RepoUtf8Path::access_dir().await?;

        Ok(Self {
            dbs: DashMap::<Utf8PathBuf, TuringDB>::new(),
            repo_dir: path,
        })
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
            let database_name = TuringEngine::to_utf8(database_name_raw)?;

            if database_entry.file_type().await?.is_dir() {
                let mut repo = async_fs::read_dir(&database_entry.path()).await?;
                let mut current_db = TuringDB::default();

                while let Some(document_entry) = repo.try_next().await? {
                    let mut field_keys = Vec::new();

                    if document_entry.file_type().await?.is_dir() {
                        let document_name_raw = document_entry.file_name();
                        let document_name = TuringEngine::to_utf8(document_name_raw)?;

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

        Ok(OpsOutcome::RepoInitialized)
    }

    fn to_utf8(value: OsString) -> Result<Utf8PathBuf, TuringDbError> {
        match value.as_os_str().to_str() {
            None => Err(TuringDbError::InvalidPathUnicodeName),
            Some(path_str) => {
                let mut path: Utf8PathBuf = Utf8PathBuf::new();
                path.push(path_str);
                Ok(path)
            }
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

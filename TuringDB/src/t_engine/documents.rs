use async_lock::Mutex;

/// #### Contains an in-memory representation of a document, with an async lock on sled file descriptor and document keys
/// ```
/// #[derive(Debug, Clone)]
/// pub (crate)struct Document {
///     fd: Mutex<sled::Db>,
///     keys: Vec<String>
/// }
/// ```
#[derive(Debug)]
pub(crate) struct Document {
    pub(crate) fd: Mutex<sled::Db>,
    pub(crate) keys: Vec<Vec<u8>>,
}
/*
impl Document {

    /// Create a document
    pub async fn doc_create(&self, db_name: &Path, doc_name: &Path) -> Result<DbOps> {
        if self.dbs.is_empty() {
            return Ok(DbOps::RepoEmpty);
        }

        if let Some(path) = doc_name.to_str() {
            if path.is_empty() {
                return Ok(DbOps::EncounteredErrors(
                    "[TuringDB::<DocumentCreate>::(ERROR)-DOCUMENT_NAME_EMPTY]".to_owned(),
                ));
            }
        }

        if let Some(mut database) = self.dbs.get_mut(&OsString::from(db_name)) {
            let mut path: PathBuf = REPO_NAME.into();
            path.push(db_name);
            path.push(doc_name);

            if database.list.get_mut(&OsString::from(doc_name)).is_some() {
                Ok(DbOps::DocumentAlreadyExists)
            } else {
                database.value_mut().list.insert(
                    OsString::from(doc_name),
                    Document {
                        fd: Mutex::new(sled::Config::default().create_new(true).path(path).open()?),
                        keys: Vec::new(),
                    },
                );

                Ok(DbOps::DocumentCreated)
            }
        } else {
            Ok(DbOps::DbNotFound)
        }
    }
    /// Drop a document
    pub async fn doc_drop(&self, db_name: &Path, doc_name: &Path) -> Result<DbOps> {
        if self.dbs.is_empty() {
            return Ok(DbOps::RepoEmpty);
        }

        if let Some(mut database) = self.dbs.get_mut(&OsString::from(db_name)) {
            match database.value_mut().list.remove(&OsString::from(doc_name)) {
                Some(_) => {
                    let mut path: PathBuf = REPO_NAME.into();
                    path.push(db_name);
                    path.push(doc_name);
                    async_fs::remove_dir_all(path).await?;

                    Ok(DbOps::DocumentDropped)
                }
                None => Ok(DbOps::DocumentNotFound),
            }
        } else {
            Ok(DbOps::DbNotFound)
        }
    }
    /// List all fields in a document
    pub async fn doc_list(&self, db_name: &Path) -> DbOps {
        if self.dbs.is_empty() {
            return DbOps::RepoEmpty;
        }

        if let Some(database) = self.dbs.get(&OsString::from(db_name)) {
            let list = database
                .list
                .keys()
                .map(|document| document.to_string_lossy().to_string())
                .collect::<Vec<String>>();

            if list.is_empty() {
                DbOps::DbEmpty
            } else {
                DbOps::DocumentList(list)
            }
        } else {
            DbOps::DbNotFound
        }
    }
    /// Flush all dirty I/O buffers from pagecache to disk.
    /// `RECOMMENDED:` Always use this function whenever you are building a networked server
    pub async fn flush(&self, db_name: &Path, doc_name: &Path) -> Result<DbOps> {
        if let Some(mut database) = self.dbs.get_mut(&OsString::from(db_name)) {
            if let Some(document) = database.value_mut().list.get_mut(&OsString::from(doc_name)) {
                document.fd.lock().await.flush()?;
                Ok(DbOps::Commited)
            } else {
                Ok(DbOps::DocumentNotFound)
            }
        } else {
            Ok(DbOps::DbNotFound)
        }
    }
}
*/

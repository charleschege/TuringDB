use crate::{Document, OpsOutcome, TuringDbError, TuringResult};
use async_fs::DirBuilder;
use async_lock::Mutex;
use camino::{Utf8Path, Utf8PathBuf};
use std::collections::hash_map::HashMap;

/// #### Contains the list of documents and databases in-memory
/// ```
/// #[derive(Debug, Clone)]
/// struct TuringDB {
///     list: HashMap<Utf8Utf8PathBuf, Document>,
/// }
///```
#[derive(Debug)]
pub(crate) struct TuringDB {
    pub(crate) list: HashMap<Utf8PathBuf, Document>,
}

impl TuringDB {
    /// Create a new in-memory database
    pub(crate) fn new() -> Self {
        Self {
            list: { HashMap::default() },
        }
    }

    /// Create a database
    pub(crate) async fn db_create(
        mut self,
        repo_dir: &Utf8Path,
        db_name: &Utf8Path,
    ) -> Result<OpsOutcome, TuringDbError> {
        let path = Self::build_path(repo_dir, db_name);
        DirBuilder::new().recursive(false).create(path).await?;

        let new_document = sled::Config::default()
            .path("temp")
            .temporary(true)
            .open()?;

        self.list.insert(db_name.into(), new_document);

        Ok(OpsOutcome::DbCreated)
    }

    /// Drop the database
    pub async fn db_drop(
        &self,
        repo_dir: &Utf8Path,
        db_name: &Utf8Path,
    ) -> Result<OpsOutcome, TuringDbError> {
        let path = Self::build_path(repo_dir, db_name);
        async_fs::remove_dir_all(path).await?;

        Ok(OpsOutcome::DbDropped)
    }
    /// List all the documents in the repo
    pub fn document_list(db: &Self) -> OpsOutcome {
        let mut list: Vec<Utf8PathBuf> = Vec::new();

        db.list.iter().for_each(|document_name| {
            list.push(document_name.0.into());
        });

        if list.is_empty() {
            OpsOutcome::DbEmpty
        } else {
            OpsOutcome::DocumentList(list)
        }
    }
    /// List all documents in a database sorted alphabetically
    //TODO Check if uppercase and lowercase and other characters appear sorted
    pub fn document_list_sorted(db: &Self) -> OpsOutcome {
        let mut list: Vec<Utf8PathBuf> = Vec::new();

        db.list.iter().for_each(|document_name| {
            list.push(document_name.0.into());
        });

        list.sort();

        if list.is_empty() {
            OpsOutcome::DbEmpty
        } else {
            OpsOutcome::DocumentList(list)
        }
    }
    /// Create a new document
    pub async fn document_create(
        &mut self,
        repo_dir: &Utf8Path,
        db_name: &Utf8Path,
        document_name: &Utf8Path,
    ) -> TuringResult<OpsOutcome> {
        match self.list.get(document_name) {
            Some(_) => Err(TuringDbError::AlreadyExists),
            None => {
                let path = TuringDB::build_document_path(repo_dir, db_name, document_name);

                let document = sled::Config::default()
                    .create_new(false)
                    .path(&path)
                    .open()?;

                self.list.insert(document_name.to_path_buf(), document);

                Ok(OpsOutcome::DocumentCreated)
            }
        }
    }
    /// Drop a document
    pub async fn document_drop(
        &mut self,
        repo_dir: &Utf8Path,
        db_name: &Utf8Path,
        document_name: &Utf8Path,
    ) -> TuringResult<OpsOutcome> {
        let path = TuringDB::build_document_path(repo_dir, db_name, document_name);

        async_fs::remove_dir_all(path).await?;

        self.list.remove(document_name);

        Ok(OpsOutcome::DocumentDropped)
    }

    fn build_path(repo_dir: &Utf8Path, db_name: &Utf8Path) -> Utf8PathBuf {
        let mut path: Utf8PathBuf = repo_dir.into();
        path.push(db_name);

        path
    }

    fn build_document_path(
        repo_dir: &Utf8Path,
        db_name: &Utf8Path,
        document_name: &Utf8Path,
    ) -> Utf8PathBuf {
        let mut path: Utf8PathBuf = repo_dir.into();
        path.push(db_name);
        path.push(document_name);

        path
    }
}

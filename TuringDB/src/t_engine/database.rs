use crate::{Document, OpsOutcome, TuringDbError};
use async_fs::DirBuilder;
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
        let new_document = Document::new(&repo_dir.into()).await?;
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
    pub fn document_list(&self) -> OpsOutcome {
        let list = self
            .list
            .iter()
            .map(|db| db.0.clone())
            .collect::<Vec<Utf8PathBuf>>();

        if list.is_empty() {
            OpsOutcome::RepoEmpty
        } else {
            OpsOutcome::DbList(list)
        }
    }

    fn build_path(repo_dir: &Utf8Path, db_name: &Utf8Path) -> Utf8PathBuf {
        let mut path: Utf8PathBuf = repo_dir.into();
        path.push(db_name);

        path
    }
}

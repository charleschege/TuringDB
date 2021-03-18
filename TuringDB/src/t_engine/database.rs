use crate::Document;
use camino::Utf8PathBuf;
use std::collections::HashMap;
/// #### Contains the list of documents and databases in-memory
/// ```
/// #[derive(Debug, Clone)]
/// struct Tdb {
///     list: HashMap<OsString, Document>,
/// }
///```
#[derive(Debug)]
pub(crate) struct TuringDB {
    pub(crate) list: HashMap<Utf8PathBuf, Document>,
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

impl Default for TuringDB {
    /// Create a new in-memory database
    fn default() -> Self {
        Self {
            list: HashMap::default(),
        }
    }
}
/*
impl TuringDB {

    /// Create a database
    pub async fn db_create(&self, db_name: &Path) -> Result<DbOps> {
        let mut path: PathBuf = REPO_NAME.into();
        path.push(db_name);

        DirBuilder::new().recursive(false).create(path).await?;

        self.dbs.insert(db_name.into(), Tdb::new());

        Ok(DbOps::DbCreated)
    }
    /// Drop the database
    pub async fn db_drop(&self, db_name: &Path) -> Result<DbOps> {
        if self.dbs.is_empty() {
            return Ok(DbOps::RepoEmpty);
        }

        let mut path: PathBuf = REPO_NAME.into();
        path.push(db_name);
        async_fs::remove_dir_all(path).await?;

        self.dbs.remove(&OsString::from(db_name));

        Ok(DbOps::DbDropped)
    }
    /// List all the databases in the repo
    pub async fn db_list(&self) -> DbOps {
        if self.dbs.is_empty() {
            return DbOps::RepoEmpty;
        }

        let list = self
            .dbs
            .iter()
            .map(|db| db.key().clone().to_string_lossy().to_string())
            .collect::<Vec<String>>();

        if list.is_empty() {
            DbOps::RepoEmpty
        } else {
            DbOps::DbList(list)
        }
    }



    /************** DATABASES *******************/
    /// Create a database
    pub async fn db_create(&self, db_name: &Path) -> Result<DbOps> {
        let mut path: PathBuf = REPO_NAME.into();
        path.push(db_name);

        DirBuilder::new().recursive(false).create(path).await?;

        self.dbs.insert(db_name.into(), Tdb::new());

        Ok(DbOps::DbCreated)
    }
    /// Drop the database
    pub async fn db_drop(&self, db_name: &Path) -> Result<DbOps> {
        if self.dbs.is_empty() {
            return Ok(DbOps::RepoEmpty);
        }

        let mut path: PathBuf = REPO_NAME.into();
        path.push(db_name);
        async_fs::remove_dir_all(path).await?;

        self.dbs.remove(&OsString::from(db_name));

        Ok(DbOps::DbDropped)
    }
    /// List all the databases in the repo
    pub async fn db_list(&self) -> DbOps {
        if self.dbs.is_empty() {
            return DbOps::RepoEmpty;
        }

        let list = self
            .dbs
            .iter()
            .map(|db| db.key().clone().to_string_lossy().to_string())
            .collect::<Vec<String>>();

        if list.is_empty() {
            DbOps::RepoEmpty
        } else {
            DbOps::DbList(list)
        }
    }
}*/

use crate::errors::format_error;
use async_dup::Arc;
use custom_codes::{DbOps, DownCastErrors};
use turingdb::TuringEngine;
use turingdb_helpers::TuringOp;
use std::path::Path;
/// Handles database queries
/// ```rust
/// pub(crate) struct DbQuery;
/// ```
pub(crate) struct DbQuery;

impl DbQuery {
    /// ### Gets a list of all databases in a repo
    /// #### Usage
    /// ```rust
    /// use crate::DatabaseQuery;
    /// use turingdb::TuringEngine;
    ///
    /// let foo = TuringEngine::new();
    /// foo.repo_init().await;
    /// // Start an async runtime
    ///     |- let foo = Arc::new(&foo); // This `Arc` must be from a module supporting async
    ///     |-  // spawn a task
    ///             |- let foo = Arc::clone(&foo);
    ///             |- DatabaseQuery::list(&foo);
    /// ```
    pub async fn list(storage: Arc<TuringEngine>) -> DbOps {
        storage.db_list().await
    }
    /// ### Create a database in a repo
    ///
    /// This function also takes an array of bytes `&[u8]` as a parameter;
    /// This array of bytes must be able to deserialize into a database name `&str` using `std::str::from_utf8(value)`
    ///
    /// #### Usage
    /// ```rust
    /// use crate::DatabaseQuery;
    /// use turingdb::TuringEngine;
    ///
    /// let foo = TuringEngine::new();
    /// foo.repo_init().await;
    /// // Start an async runtime
    ///     |- let foo = Arc::new(&foo); // This `Arc` must be from a module supporting async
    ///     |-  // spawn a task
    ///             |- let foo = Arc::clone(&foo);
    ///             |- DatabaseQuery::create(&foo, &[data_to_deserialize]).await;
    /// ```
    pub async fn create(storage: Arc<TuringEngine>, value: &[u8]) -> DbOps {
        if value.is_empty() == true {
            return DbOps::EncounteredErrors(
                "[TuringDB::<DbCreate>::(ERROR)-MISSING_DB_NAME]".to_owned(),
            );
        }

        let db_name = match std::str::from_utf8(value) {
            Ok(value) => value,
            Err(e) => return format_error(&TuringOp::DbCreate, &anyhow::Error::new(e)),
        };

        match storage.db_create(&Path::new(db_name)).await {
            Ok(op_result) => op_result,
            Err(e) => match custom_codes::try_downcast(&e) {
                DownCastErrors::AlreadyExists => DbOps::DbAlreadyExists,
                DownCastErrors::NotFound => DbOps::RepoNotFound,
                DownCastErrors::PermissionDenied => DbOps::PermissionDenied,
                _ => format_error(&TuringOp::DbCreate, &e),
            },
        }
    }
    /// ### Drop a database in a repo
    ///
    /// This function also takes an array of bytes `&[u8]` as a parameter;
    /// This array of bytes must be able to deserialize into a database name `&str` using `std::str::from_utf8(value)`
    ///
    /// #### Usage
    /// ```rust
    /// use crate::DatabaseQuery;
    /// use turingdb::TuringEngine;
    ///
    /// let foo = TuringEngine::new();
    /// foo.repo_init().await;
    /// // Start an async runtime
    ///     |- let foo = Arc::new(&foo); // This `Arc` must be from a module supporting async
    ///     |-  // spawn a task
    ///             |- let foo = Arc::clone(&foo);
    ///             |- DatabaseQuery::create(&foo, &[data_to_deserialize]).await;
    /// ```
    pub async fn drop(storage: Arc<TuringEngine>, value: &[u8]) -> DbOps {
        if value.is_empty() == true {
            return DbOps::EncounteredErrors(
                "[TuringDB::<DbDrop>::(ERROR)-MISSING_DB_NAME]".to_owned(),
            );
        }

        let db_name = match std::str::from_utf8(value) {
            Ok(value) => value,
            Err(e) => return format_error(&TuringOp::DbDrop, &anyhow::Error::new(e)),
        };

        match storage.db_drop(&Path::new(db_name)).await {
            Ok(op_result) => op_result,
            Err(e) => match custom_codes::try_downcast(&e) {
                DownCastErrors::NotFound => DbOps::RepoNotFound,
                DownCastErrors::PermissionDenied => DbOps::PermissionDenied,
                _ => format_error(&TuringOp::DbCreate, &e),
            },
        }
    }
}

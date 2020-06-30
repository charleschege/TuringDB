use crate::errors::format_error;
use async_dup::Arc;
use custom_codes::{DbOps, DownCastErrors};
use turingdb::TuringEngine;
use turingdb_helpers::TuringOp;

/// Handles repository queries
/// ```rust
/// pub(crate) struct RepoQuery;
/// ```
pub(crate) struct RepoQuery;

impl RepoQuery {
    /// ### Create a new repository
    /// #### Usage
    /// ```rust
    /// use crate::RepoQuery;
    /// use turingdb::TuringEngine;
    /// 
    /// let foo = TuringEngine::new();
    /// foo.repo_init().await;
    /// // Start an async runtime
    ///     |- let foo = Arc::new(&foo); // This `Arc` must be from a module supporting async
    ///     |-  // spawn a task
    ///             |- let foo = Arc::clone(&foo);
    ///             |- RepoQuery::create(&foo).await;
    /// ```
    pub async fn create(storage: Arc<TuringEngine>) -> DbOps {
        match storage.repo_create().await {
            Ok(op_result) => op_result,
            Err(e) => match custom_codes::try_downcast(&e) {
                DownCastErrors::AlreadyExists => DbOps::RepoAlreadyExists,
                DownCastErrors::PermissionDenied => DbOps::PermissionDenied,
                _ => format_error(&TuringOp::RepoCreate, &e).await,
            },
        }
    }
    /// ### Drop an existing repository
    /// #### Usage
    /// ```rust
    /// use crate::RepoQuery;
    /// use turingdb::TuringEngine;
    /// 
    /// let foo = TuringEngine::new();
    /// foo.repo_init().await;
    /// // Start an async runtime
    ///     |- let foo = Arc::new(&foo); // This `Arc` must be from a module supporting async
    ///     |-  // spawn a task
    ///             |- let foo = Arc::clone(&foo);
    ///             |- RepoQuery::drop(&foo).await;
    /// ```
    pub async fn drop(storage: Arc<TuringEngine>) -> DbOps {
        match storage.repo_drop().await {
            Ok(op_result) => op_result,
            Err(e) => match custom_codes::try_downcast(&e) {
                DownCastErrors::NotFound => DbOps::RepoNotFound,
                DownCastErrors::PermissionDenied => DbOps::PermissionDenied,
                _ => format_error(&TuringOp::RepoDrop, &e).await,
            },
        }
    }
}

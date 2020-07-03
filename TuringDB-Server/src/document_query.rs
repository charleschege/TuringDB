use crate::errors::format_error;
use async_dup::Arc;
use custom_codes::{DbOps, DownCastErrors};
use serde::{Deserialize, Serialize};
use turingdb::TuringEngine;
use turingdb_helpers::TuringOp;

/// Handles database queries
/// ```rust
/// #[derive(Debug, Serialize, Deserialize)]
/// pub(crate) struct DocumentQuery {
///     db: String,
///     document: Option<String>,
/// }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct DocumentQuery {
    db: String,
    document: Option<String>,
}

impl DocumentQuery {
    /// ### Create a document in a database
    ///
    /// This function also takes an array of bytes `&[u8]` as a parameter;
    /// This array of bytes must be able to deserialize into a `crate::DocumentQuery` struct  using bincode
    ///
    /// #### Usage
    /// ```rust
    /// use crate::DocumentQuery;
    /// use turingdb::TuringEngine;
    ///
    /// let foo = TuringEngine::new();
    /// foo.repo_init().await;
    /// // Start an async runtime
    ///     |- let foo = Arc::new(&foo); // This `Arc` must be from a module supporting async
    ///     |-  // spawn a task
    ///             |- let foo = Arc::clone(&foo);
    ///             |- DocumentQuery::create(&foo, &[data_to_deserialize]).await;
    /// ```
    pub async fn create(storage: Arc<TuringEngine>, value: &[u8]) -> DbOps {
        if value.is_empty() == true {
            return DbOps::EncounteredErrors(
                "[TuringDB::<DocumentCreate>::(ERROR)-GOOD_HEADER_NO_DATA]".to_owned(),
            );
        }

        let deser_document = match bincode::deserialize::<DocumentQuery>(value) {
            Ok(value) => value,
            Err(e) => return format_error(&TuringOp::DocumentCreate, &anyhow::Error::new(e)),
        };

        let doc_check = match deser_document.document {
            Some(document) => document,
            None => {
                return DbOps::EncounteredErrors(
                    "[TuringDB::<DocumentCreate>::(ERROR)-DOCUMENT_NAME_NOT_PROVIDED]".to_owned(),
                )
            }
        };

        match storage.doc_create(&deser_document.db, &doc_check).await {
            Ok(op_result) => op_result,
            Err(e) => match custom_codes::try_downcast(&e) {
                DownCastErrors::AlreadyExists => DbOps::DocumentAlreadyExists,
                DownCastErrors::NotFound => DbOps::RepoNotFound,
                DownCastErrors::PermissionDenied => DbOps::PermissionDenied,
                _ => format_error(&TuringOp::DocumentCreate, &e),
            },
        }
    }
    /// ### List all documents in a database
    ///
    /// This function also takes an array of bytes `&[u8]` as a parameter;
    /// This array of bytes must be able to deserialize into a `crate::DocumentQuery` struct  using bincode
    ///
    /// #### Usage
    /// ```rust
    /// use crate::DocumentQuery;
    /// use turingdb::TuringEngine;
    ///
    /// let foo = TuringEngine::new();
    /// foo.repo_init().await;
    /// // Start an async runtime
    ///     |- let foo = Arc::new(&foo); // This `Arc` must be from a module supporting async
    ///     |-  // spawn a task
    ///             |- let foo = Arc::clone(&foo);
    ///             |- DocumentQuery::list(&foo, &[data_to_deserialize]).await;
    /// ```
    pub async fn list(storage: Arc<TuringEngine>, value: &[u8]) -> DbOps {
        if value.is_empty() == true {
            return DbOps::EncounteredErrors(
                "[TuringDB::<DbList>::(ERROR)-GOOD_HEADER_NO_DATA]".to_owned(),
            );
        }

        let deser_document = match bincode::deserialize::<DocumentQuery>(value) {
            Ok(value) => value,
            Err(e) => return format_error(&TuringOp::DocumentList, &anyhow::Error::new(e)),
        };

        match deser_document.document {
            Some(_) => {
                return DbOps::EncounteredErrors(
                    "[TuringDB::<DocumentList>::(ERROR)-QUERY_ARGS_EXCEEDED]".to_owned(),
                )
            }
            None => (),
        };

        storage.doc_list(&deser_document.db).await
    }
    /// ### Drops a document in a database
    ///
    /// This function also takes an array of bytes `&[u8]` as a parameter;
    /// This array of bytes must be able to deserialize into a `crate::DocumentQuery` struct  using bincode
    ///
    /// #### Usage
    /// ```rust
    /// use crate::DocumentQuery;
    /// use turingdb::TuringEngine;
    ///
    /// let foo = TuringEngine::new();
    /// foo.repo_init().await;
    /// // Start an async runtime
    ///     |- let foo = Arc::new(&foo); // This `Arc` must be from a module supporting async
    ///     |-  // spawn a task
    ///             |- let foo = Arc::clone(&foo);
    ///             |- DocumentQuery::drop(&foo, &[data_to_deserialize]).await;
    /// ```
    pub async fn drop(storage: Arc<TuringEngine>, value: &[u8]) -> DbOps {
        if value.is_empty() == true {
            return DbOps::EncounteredErrors(
                "[TuringDB::<DbDrop>::(ERROR)-GOOD_HEADER_NO_DATA]".to_owned(),
            );
        }

        let deser_document = match bincode::deserialize::<DocumentQuery>(value) {
            Ok(value) => value,
            Err(e) => return format_error(&TuringOp::DocumentDrop, &anyhow::Error::new(e)),
        };

        let doc_check = match deser_document.document {
            Some(document) => document,
            None => {
                return DbOps::EncounteredErrors(
                    "[TuringDB::<DocumentDrop>::(ERROR)-DOCUMENT_NAME_NOT_PROVIDED]".to_owned(),
                )
            }
        };

        match storage.doc_drop(&deser_document.db, &doc_check).await {
            Ok(op_result) => op_result,
            Err(e) => match custom_codes::try_downcast(&e) {
                DownCastErrors::NotFound => DbOps::RepoNotFound,
                DownCastErrors::PermissionDenied => DbOps::PermissionDenied,
                _ => format_error(&TuringOp::DocumentDrop, &e),
            },
        }
    }
}

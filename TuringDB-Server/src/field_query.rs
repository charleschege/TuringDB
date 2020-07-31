use crate::errors::format_error;
use async_dup::Arc;
use custom_codes::{DbOps, DownCastErrors};
use serde::{Deserialize, Serialize};
use std::path::Path;
use turingdb::TuringEngine;
use turingdb_helpers::TuringOp;

/// Handles database queries
/// ```rust
/// #[derive(Debug, Serialize, Deserialize)]
/// pub(crate) struct FieldQuery {
///     db: String,
///     document: String,
///     field: String,
///     payload: Option<Vec<u8>>,
/// }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct FieldQuery {
    db: String,
    document: String,
    field: String,
    payload: Option<Vec<u8>>,
}

impl FieldQuery {
    /// ### List all fields in a document
    ///
    /// This function also takes an array of bytes `&[u8]` as a parameter;
    /// This array of bytes must be able to deserialize into a `crate::FieldQuery` struct  using bincode
    ///
    /// #### Usage
    /// ```rust
    /// use crate::FieldQuery;
    /// use turingdb::TuringEngine;
    ///
    /// let foo = TuringEngine::new();
    /// foo.repo_init().await;
    /// // Start an async runtime
    ///     |- let foo = Arc::new(&foo); // This `Arc` must be from a module supporting async
    ///     |-  // spawn a task
    ///             |- let foo = Arc::clone(&foo);
    ///             |- FieldQuery::list(&foo, &[data_to_deserialize]).await;
    /// ```
    pub async fn list(storage: Arc<TuringEngine>, value: &[u8]) -> DbOps {
        if value.is_empty() == true {
            return DbOps::EncounteredErrors(
                "[TuringDB::<FieldList>::(ERROR)-GOOD_HEADER_NO_DATA]".to_owned(),
            );
        }

        let deser_document = match bincode::deserialize::<FieldQuery>(value) {
            Ok(value) => value,
            Err(e) => return format_error(&TuringOp::FieldList, &anyhow::Error::new(e)),
        };

        match deser_document.payload {
            Some(_) => {
                return DbOps::EncounteredErrors(
                    "[TuringDB::<FieldList>::(ERROR)-QUERY_ARGS_EXCEEDED]".to_owned(),
                )
            }
            None => (),
        };

        storage
            .field_list(
                &Path::new(&deser_document.db),
                &Path::new(&deser_document.document),
            )
            .await
    }
    /// ### Insert key/value in a document, failing if the key already exists
    ///
    /// This function also takes an array of bytes `&[u8]` as a parameter;
    /// This array of bytes must be able to deserialize into a `crate::FieldQuery` struct  using bincode
    ///
    /// #### Usage
    /// ```rust
    /// use crate::FieldQuery;
    /// use turingdb::TuringEngine;
    ///
    /// let foo = TuringEngine::new();
    /// foo.repo_init().await;
    /// // Start an async runtime
    ///     |- let foo = Arc::new(&foo); // This `Arc` must be from a module supporting async
    ///     |-  // spawn a task
    ///             |- let foo = Arc::clone(&foo);
    ///             |- FieldQuery::insert(&foo, &[data_to_deserialize]).await;
    /// ```
    pub async fn insert(storage: Arc<TuringEngine>, value: &[u8]) -> DbOps {
        if value.is_empty() == true {
            return DbOps::EncounteredErrors(
                "[TuringDB::<FieldInsert>::(ERROR)-GOOD_HEADER_NO_DATA]".to_owned(),
            );
        }

        let deser_document = match bincode::deserialize::<FieldQuery>(value) {
            Ok(value) => value,
            Err(e) => return format_error(&TuringOp::FieldInsert, &anyhow::Error::new(e)),
        };

        let data_check = match deser_document.payload {
            Some(document) => document,
            None => {
                return DbOps::EncounteredErrors(
                    "[TuringDB::<FieldInsert>::(ERROR)-FIELD_PAYLOAD_NOT_PROVIDED]".to_owned(),
                )
            }
        };

        match storage
            .field_insert(
                &Path::new(&deser_document.db),
                &Path::new(&deser_document.document),
                &deser_document.field.as_bytes(),
                &data_check,
            )
            .await
        {
            Ok(op_result) => {
                match storage
                    .flush(
                        Path::new(&deser_document.db),
                        Path::new(&deser_document.document),
                    )
                    .await
                {
                    Ok(_) => op_result,
                    Err(e) => DbOps::EncounteredErrors(e.to_string()),
                }
            }
            Err(e) => match custom_codes::try_downcast(&e) {
                DownCastErrors::NotFound => DbOps::RepoNotFound,
                DownCastErrors::PermissionDenied => DbOps::PermissionDenied,
                _ => format_error(&TuringOp::FieldInsert, &e),
            },
        }
    }
    /// ### get a field value in a document using its `key`
    ///
    /// This function also takes an array of bytes `&[u8]` as a parameter;
    /// This array of bytes must be able to deserialize into a `crate::FieldQuery` struct  using bincode
    ///
    /// #### Usage
    /// ```rust
    /// use crate::FieldQuery;
    /// use turingdb::TuringEngine;
    ///
    /// let foo = TuringEngine::new();
    /// foo.repo_init().await;
    /// // Start an async runtime
    ///     |- let foo = Arc::new(&foo); // This `Arc` must be from a module supporting async
    ///     |-  // spawn a task
    ///             |- let foo = Arc::clone(&foo);
    ///             |- FieldQuery::get(&foo, &[data_to_deserialize]).await;
    /// ```
    pub async fn get(storage: Arc<TuringEngine>, value: &[u8]) -> DbOps {
        if value.is_empty() == true {
            return DbOps::EncounteredErrors(
                "[TuringDB::<FieldGet>::(ERROR)-GOOD_HEADER_NO_DATA]".to_owned(),
            );
        }

        let deser_document = match bincode::deserialize::<FieldQuery>(value) {
            Ok(value) => value,
            Err(e) => return format_error(&TuringOp::FieldGet, &anyhow::Error::new(e)),
        };

        match deser_document.payload {
            Some(_) => {
                return DbOps::EncounteredErrors(
                    "[TuringDB::<FieldGet>::(ERROR)-QUERY_ARGS_EXCEEDED]".to_owned(),
                )
            }
            None => (),
        };

        match storage
            .field_get(
                &Path::new(&deser_document.db),
                &Path::new(&deser_document.document),
                &deser_document.field.as_bytes(),
            )
            .await
        {
            Ok(op_result) => op_result,
            Err(e) => match custom_codes::try_downcast(&e) {
                DownCastErrors::NotFound => DbOps::RepoNotFound,
                DownCastErrors::PermissionDenied => DbOps::PermissionDenied,
                _ => format_error(&TuringOp::FieldGet, &e),
            },
        }
    }
    /// ### Remove a field in a document based on its `key`
    ///
    /// This function also takes an array of bytes `&[u8]` as a parameter;
    /// This array of bytes must be able to deserialize into a `crate::FieldQuery` struct  using bincode
    ///
    /// #### Usage
    /// ```rust
    /// use crate::FieldQuery;
    /// use turingdb::TuringEngine;
    ///
    /// let foo = TuringEngine::new();
    /// foo.repo_init().await;
    /// // Start an async runtime
    ///     |- let foo = Arc::new(&foo); // This `Arc` must be from a module supporting async
    ///     |-  // spawn a task
    ///             |- let foo = Arc::clone(&foo);
    ///             |- FieldQuery::remove(&foo, &[data_to_deserialize]).await;
    /// ```
    pub async fn remove(storage: Arc<TuringEngine>, value: &[u8]) -> DbOps {
        if value.is_empty() == true {
            return DbOps::EncounteredErrors(
                "[TuringDB::<FieldRemove>::(ERROR)-GOOD_HEADER_NO_DATA]".to_owned(),
            );
        }

        let deser_document = match bincode::deserialize::<FieldQuery>(value) {
            Ok(value) => value,
            Err(e) => return format_error(&TuringOp::FieldRemove, &anyhow::Error::new(e)),
        };

        match deser_document.payload {
            Some(_) => {
                return DbOps::EncounteredErrors(
                    "[TuringDB::<FieldRemove>::(ERROR)-QUERY_ARGS_EXCEEDED]".to_owned(),
                )
            }
            None => (),
        };

        match storage
            .field_remove(
                &Path::new(&deser_document.db),
                &Path::new(&deser_document.document),
                &deser_document.field.as_bytes(),
            )
            .await
        {
            Ok(op_result) => {
                match storage
                    .flush(
                        &Path::new(&deser_document.db),
                        &Path::new(&deser_document.document),
                    )
                    .await
                {
                    Ok(_) => op_result,
                    Err(e) => DbOps::EncounteredErrors(e.to_string()),
                }
            }
            Err(e) => match custom_codes::try_downcast(&e) {
                DownCastErrors::NotFound => DbOps::RepoNotFound,
                DownCastErrors::PermissionDenied => DbOps::PermissionDenied,
                _ => format_error(&TuringOp::FieldRemove, &e),
            },
        }
    }
    /// ### Update the `value` contents all a `key` in a field
    ///
    /// This function also takes an array of bytes `&[u8]` as a parameter;
    /// This array of bytes must be able to deserialize into a `crate::FieldQuery` struct  using bincode
    ///
    /// #### Usage
    /// ```rust
    /// use crate::FieldQuery;
    /// use turingdb::TuringEngine;
    ///
    /// let foo = TuringEngine::new();
    /// foo.repo_init().await;
    /// // Start an async runtime
    ///     |- let foo = Arc::new(&foo); // This `Arc` must be from a module supporting async
    ///     |-  // spawn a task
    ///             |- let foo = Arc::clone(&foo);
    ///             |- FieldQuery::modify(&foo, &[data_to_deserialize]).await;
    /// ```
    pub async fn modify(storage: Arc<TuringEngine>, value: &[u8]) -> DbOps {
        if value.is_empty() == true {
            return DbOps::EncounteredErrors(
                "[TuringDB::<FieldModify>::(ERROR)-GOOD_HEADER_NO_DATA]".to_owned(),
            );
        }

        let deser_document = match bincode::deserialize::<FieldQuery>(value) {
            Ok(value) => value,
            Err(e) => return format_error(&TuringOp::FieldModify, &anyhow::Error::new(e)),
        };

        let data_check = match deser_document.payload {
            Some(document) => document,
            None => {
                return DbOps::EncounteredErrors(
                    "[TuringDB::<FieldModify>::(ERROR)-FIELD_PAYLOAD_NOT_PROVIDED]".to_owned(),
                )
            }
        };

        match storage
            .field_modify(
                &Path::new(&deser_document.db),
                &Path::new(&deser_document.document),
                &deser_document.field.as_bytes(),
                &data_check,
            )
            .await
        {
            Ok(op_result) => {
                match storage
                    .flush(
                        Path::new(&deser_document.db),
                        Path::new(&deser_document.document),
                    )
                    .await
                {
                    Ok(_) => op_result,
                    Err(e) => DbOps::EncounteredErrors(e.to_string()),
                }
            }
            Err(e) => match custom_codes::try_downcast(&e) {
                DownCastErrors::NotFound => DbOps::RepoNotFound,
                DownCastErrors::PermissionDenied => DbOps::PermissionDenied,
                _ => format_error(&TuringOp::FieldModify, &e),
            },
        }
    }
}

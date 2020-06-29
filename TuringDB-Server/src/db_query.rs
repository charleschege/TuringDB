use crate::errors::format_error;
use async_dup::Arc;
use custom_codes::{DbOps, DownCastErrors};
use turingdb::TuringEngine;
use turingdb_helpers::TuringOp;

pub(crate) struct DbQuery;

impl DbQuery {
    pub async fn list(storage: Arc<TuringEngine>) -> DbOps {
        storage.db_list().await
    }

    pub async fn create(storage: Arc<TuringEngine>, value: &[u8]) -> DbOps {
        if value.is_empty() == true {
            return DbOps::EncounteredErrors(
                "[TuringDB::<DbCreate>::(ERROR)-MISSING_DB_NAME]".to_owned(),
            );
        }

        let db_name = match std::str::from_utf8(value) {
            Ok(value) => value,
            Err(e) => return format_error(&TuringOp::DbCreate, &anyhow::Error::new(e)).await,
        };

        match storage.db_create(db_name).await {
            Ok(op_result) => op_result,
            Err(e) => match custom_codes::try_downcast(&e) {
                DownCastErrors::AlreadyExists => DbOps::DbAlreadyExists,
                DownCastErrors::NotFound => DbOps::RepoNotFound,
                DownCastErrors::PermissionDenied => DbOps::PermissionDenied,
                _ => format_error(&TuringOp::DbCreate, &e).await,
            },
        }
    }
    pub async fn drop(storage: Arc<TuringEngine>, value: &[u8]) -> DbOps {
        if value.is_empty() == true {
            return DbOps::EncounteredErrors(
                "[TuringDB::<DbDrop>::(ERROR)-MISSING_DB_NAME]".to_owned(),
            );
        }

        let db_name = match std::str::from_utf8(value) {
            Ok(value) => value,
            Err(e) => return format_error(&TuringOp::DbDrop, &anyhow::Error::new(e)).await,
        };

        match storage.db_drop(db_name).await {
            Ok(op_result) => op_result,
            Err(e) => match custom_codes::try_downcast(&e) {
                DownCastErrors::NotFound => DbOps::RepoNotFound,
                DownCastErrors::PermissionDenied => DbOps::PermissionDenied,
                _ => format_error(&TuringOp::DbCreate, &e).await,
            },
        }
    }
}

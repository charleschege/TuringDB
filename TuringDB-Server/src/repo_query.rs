use turingdb::TuringEngine;
use async_dup::Arc;
use custom_codes::{DownCastErrors, DbOps};
use crate::commands::TuringOp;
use crate::errors::format_error;

pub (crate) struct RepoQuery;

impl RepoQuery {
    pub async fn create(storage: Arc<TuringEngine>) -> DbOps {

        match storage.repo_create().await {
            Ok(op_result) => op_result,
            Err(e) => {

                match custom_codes::try_downcast(&e) {
                    DownCastErrors::AlreadyExists => DbOps::RepoAlreadyExists,
                    DownCastErrors::PermissionDenied => DbOps::PermissionDenied,
                    _ => format_error(&TuringOp::RepoCreate, &e).await,
                }
            }
        }
    }
    pub async fn drop(storage: Arc<TuringEngine>) -> DbOps {

        match storage.repo_drop().await {
            Ok(op_result) => op_result,
            Err(e) => {

                match custom_codes::try_downcast(&e) {
                    DownCastErrors::NotFound => DbOps::RepoNotFound,
                    DownCastErrors::PermissionDenied => DbOps::PermissionDenied,
                    _ => format_error(&TuringOp::RepoDrop, &e).await,
                }
            }
        }
    }
}

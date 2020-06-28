use async_dup::Arc;
use custom_codes::{DownCastErrors, DbOps};
use turingdb::TuringEngine;
use serde::{Serialize, Deserialize};

use crate::{
    commands::TuringOp,
    errors::format_error,
};

#[derive(Debug, Serialize, Deserialize)]
pub (crate) struct FieldQuery {
    db: String,
    document: String,
    field: String,
    payload: Option<Vec<u8>>,
}


impl FieldQuery {
    pub async fn list(storage: Arc<TuringEngine>, value: &[u8]) -> DbOps {
        if value.is_empty() == true {
            return DbOps::EncounteredErrors("[TuringDB::<FieldList>::(ERROR)-GOOD_HEADER_NO_DATA]".to_owned())
        }

        let deser_document = match bincode::deserialize::<FieldQuery>(value) {
            Ok(value) => value,
            Err(e) => return format_error(&TuringOp::FieldList, &anyhow::Error::new(e)).await,
        };

        match deser_document.payload {
            Some(_) => return DbOps::EncounteredErrors("[TuringDB::<FieldList>::(ERROR)-QUERY_ARGS_EXCEEDED]".to_owned()),
            None => (),
        };

        storage.field_list(&deser_document.db, &deser_document.document).await
    }
    pub async fn insert(storage: Arc<TuringEngine>, value: &[u8]) -> DbOps {
        if value.is_empty() == true {
            return DbOps::EncounteredErrors("[TuringDB::<FieldInsert>::(ERROR)-GOOD_HEADER_NO_DATA]".to_owned())
        }

        let deser_document = match bincode::deserialize::<FieldQuery>(value) {
            Ok(value) => value,
            Err(e) => return format_error(&TuringOp::FieldInsert, &anyhow::Error::new(e)).await,
        };

        let data_check = match deser_document.payload {
            Some(document) => document,
            None => return DbOps::EncounteredErrors("[TuringDB::<FieldInsert>::(ERROR)-FIELD_PAYLOAD_NOT_PROVIDED]".to_owned()),
        };

        match storage.field_insert(
            &deser_document.db,
            &deser_document.document,
            &deser_document.field,
            &data_check
        ).await {
            Ok(op_result) => {
                match storage.flush(&deser_document.db, &deser_document.document).await {
                    Ok(_) => op_result,
                    Err(e) => DbOps::EncounteredErrors(e.to_string()),
                }
            },
            Err(e) => {
                match custom_codes::try_downcast(&e) {
                    DownCastErrors::NotFound => DbOps::RepoNotFound,
                    DownCastErrors::PermissionDenied => DbOps::PermissionDenied,
                    _ => format_error(&TuringOp::FieldInsert, &e).await,
                }
            }
        }
    }
    pub async fn get(storage: Arc<TuringEngine>, value: &[u8]) -> DbOps {
        if value.is_empty() == true {
            return DbOps::EncounteredErrors("[TuringDB::<FieldGet>::(ERROR)-GOOD_HEADER_NO_DATA]".to_owned())
        }

        let deser_document = match bincode::deserialize::<FieldQuery>(value) {
            Ok(value) => value,
            Err(e) => return format_error(&TuringOp::FieldGet, &anyhow::Error::new(e)).await,
        };

        match deser_document.payload {
            Some(_) => return DbOps::EncounteredErrors("[TuringDB::<FieldGet>::(ERROR)-QUERY_ARGS_EXCEEDED]".to_owned()),
            None => ()
        };

        match storage.field_get(
            &deser_document.db,
            &deser_document.document,
            &deser_document.field,
        ).await {
            Ok(op_result) => op_result,
            Err(e) => {
                match custom_codes::try_downcast(&e) {
                    DownCastErrors::NotFound => DbOps::RepoNotFound,
                    DownCastErrors::PermissionDenied => DbOps::PermissionDenied,
                    _ => format_error(&TuringOp::FieldGet, &e).await,
                }
            }
        }
    }
    pub async fn remove(storage: Arc<TuringEngine>, value: &[u8]) -> DbOps {
        if value.is_empty() == true {
            return DbOps::EncounteredErrors("[TuringDB::<FieldRemove>::(ERROR)-GOOD_HEADER_NO_DATA]".to_owned())
        }

        let deser_document = match bincode::deserialize::<FieldQuery>(value) {
            Ok(value) => value,
            Err(e) => return format_error(&TuringOp::FieldRemove, &anyhow::Error::new(e)).await,
        };

        match deser_document.payload {
            Some(_) => return DbOps::EncounteredErrors("[TuringDB::<FieldRemove>::(ERROR)-QUERY_ARGS_EXCEEDED]".to_owned()),
            None => (),
        };

        match storage.field_remove(
            &deser_document.db,
            &deser_document.document,
            &deser_document.field,
        ).await {
            Ok(op_result) => {
                match storage.flush(&deser_document.db, &deser_document.document).await {
                    Ok(_) => op_result,
                    Err(e) => DbOps::EncounteredErrors(e.to_string()),
                }
            },
            Err(e) => {
                match custom_codes::try_downcast(&e) {
                    DownCastErrors::NotFound => DbOps::RepoNotFound,
                    DownCastErrors::PermissionDenied => DbOps::PermissionDenied,
                    _ => format_error(&TuringOp::FieldRemove, &e).await,
                }
            }
        }
    }
    pub async fn modify(storage: Arc<TuringEngine>, value: &[u8]) -> DbOps {
        if value.is_empty() == true {
            return DbOps::EncounteredErrors("[TuringDB::<FieldModify>::(ERROR)-GOOD_HEADER_NO_DATA]".to_owned())
        }

        let deser_document = match bincode::deserialize::<FieldQuery>(value) {
            Ok(value) => value,
            Err(e) => return format_error(&TuringOp::FieldModify, &anyhow::Error::new(e)).await,
        };

        let data_check = match deser_document.payload {
            Some(document) => document,
            None => return DbOps::EncounteredErrors("[TuringDB::<FieldModify>::(ERROR)-FIELD_PAYLOAD_NOT_PROVIDED]".to_owned()),
        };

        match storage.field_modify(
            &deser_document.db,
            &deser_document.document,
            &deser_document.field,
            &data_check
        ).await {
            Ok(op_result) => {
                match storage.flush(&deser_document.db, &deser_document.document).await {
                    Ok(_) => op_result,
                    Err(e) => DbOps::EncounteredErrors(e.to_string()),
                }
            },
            Err(e) => {
                match custom_codes::try_downcast(&e) {
                    DownCastErrors::NotFound => DbOps::RepoNotFound,
                    DownCastErrors::PermissionDenied => DbOps::PermissionDenied,
                    _ => format_error(&TuringOp::FieldModify, &e).await,
                }
            }
        }
    }
}

use crate::commands::{from_op, TuringOp};

#[derive(Debug, Clone)]
pub struct RepoQuery;

impl<'tp> RepoQuery {
    pub async fn create() -> &'tp [u8] {
        from_op(&TuringOp::RepoCreate).await
    }
    pub async fn drop() -> &'tp [u8] {
        from_op(&TuringOp::RepoDrop).await
    }
}
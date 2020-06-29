use crate::commands::{from_op, TuringOp};

#[derive(Debug, Clone)]
pub struct DbQuery {
    db: String,
}

impl<'tp> DbQuery {
    pub async fn new() -> Self {
        Self {
            db: Default::default(),
        }
    }
    pub async fn db(&mut self, name: &str) -> &Self {
        self.db = name.into();

        self
    }
    pub async fn own(&self) -> Self {

        self.to_owned()
    }
    pub async fn create(&self) ->Vec<u8> {
        let mut packet = from_op(&TuringOp::DbCreate).await.to_vec();
        packet.extend_from_slice(self.db.as_bytes());

        packet
    }
    pub async fn drop(&self) ->Vec<u8> {
        let mut packet = from_op(&TuringOp::DbDrop).await.to_vec();
        packet.extend_from_slice(self.db.as_bytes());

        packet
    }
    pub async fn list(&self) -> &'tp [u8] {
        
        from_op(&TuringOp::DbList).await
    }
}
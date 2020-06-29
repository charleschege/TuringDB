use serde::{Serialize, Deserialize};
use anyhow::Result;
use crate::commands::{from_op, TuringOp};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DocumentQuery {
    db: String,
    document: Option<String>,
}

impl DocumentQuery {
    pub async fn new() -> Self {
        Self {
            db: Default::default(),
            document: Default::default(),
        }
    }
    pub async fn db(&mut self, name: &str) -> &Self {
        self.db = name.into();

        self
    }
    pub async fn document(&mut self, name: &str) -> &Self {
        self.document = Some(name.into());

        self
    }
    pub async fn own(&self) -> Self {

        self.to_owned()
    }
    pub async fn create(&self) -> Result<Vec<u8>> {
        let mut packet = from_op(&TuringOp::DocumentCreate).await.to_vec();
        packet.extend_from_slice(self.db.as_bytes());
        
        let data = bincode::serialize::<Self>(self)?;
        packet.extend_from_slice(&data);

        Ok(packet)
    }
    pub async fn list(&self) -> Result<Vec<u8>> {
        let mut packet = from_op(&TuringOp::DocumentList).await.to_vec();
        packet.extend_from_slice(self.db.as_bytes());

        let data = bincode::serialize::<Self>(self)?;
        packet.extend_from_slice(&data);

        Ok(packet)
    }
    pub async fn drop(&self) -> Result<Vec<u8>> {
        let mut packet = from_op(&TuringOp::DocumentDrop).await.to_vec();
        packet.extend_from_slice(self.db.as_bytes());
        
        let data = bincode::serialize::<Self>(self)?;
        packet.extend_from_slice(&data);

        Ok(packet)
    }
}
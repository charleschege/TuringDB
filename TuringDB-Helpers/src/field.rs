use serde::{Serialize, Deserialize};
use anyhow::Result;
use crate::commands::{from_op, TuringOp};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FieldQuery {
    db: String,
    document: String,
    field: String,
    payload: Option<Vec<u8>>,
}

impl FieldQuery {
    pub async fn new() -> Self {
        Self {
            db: Default::default(),
            document: Default::default(),
            field: Default::default(),
            payload: Default::default(),
        }
    }
    pub async fn db(&mut self, name: &str) -> &Self {
        self.db = name.into();

        self
    }
    pub async fn document(&mut self, name: &str) -> &Self {
        self.document = name.into();

        self
    }
    pub async fn field(&mut self, name: &str) -> &Self {
        self.field = name.into();

        self
    }
    pub async fn payload(&mut self, value: &[u8]) -> &Self {
        self.payload = Some(value.into());

        self
    }
    pub async fn own(&self) -> Self {

        self.to_owned()
    }
    pub async fn create(&self) -> Result<Vec<u8>> {
        let mut packet = from_op(&TuringOp::FieldInsert).await.to_vec();
        packet.extend_from_slice(self.db.as_bytes());
        
        let data = bincode::serialize::<Self>(self)?;
        packet.extend_from_slice(&data);

        Ok(packet)
    }
    pub async fn get(&self) -> Result<Vec<u8>> {
        let mut packet = from_op(&TuringOp::FieldGet).await.to_vec();
        packet.extend_from_slice(self.db.as_bytes());
        
        let data = bincode::serialize::<Self>(self)?;
        packet.extend_from_slice(&data);

        Ok(packet)
    }
    pub async fn list(&self) -> Result<Vec<u8>> {
        let mut packet = from_op(&TuringOp::FieldList).await.to_vec();
        packet.extend_from_slice(self.db.as_bytes());
        
        let data = bincode::serialize::<Self>(self)?;
        packet.extend_from_slice(&data);

        Ok(packet)
    }
    pub async fn remove(&self) -> Result<Vec<u8>> {
        let mut packet = from_op(&TuringOp::FieldRemove).await.to_vec();
        packet.extend_from_slice(self.db.as_bytes());
        
        let data = bincode::serialize::<Self>(self)?;
        packet.extend_from_slice(&data);

        Ok(packet)
    }
    pub async fn modify(&self) -> Result<Vec<u8>> {
        let mut packet = from_op(&TuringOp::FieldModify).await.to_vec();
        packet.extend_from_slice(self.db.as_bytes());
        
        let data = bincode::serialize::<Self>(self)?;
        packet.extend_from_slice(&data);

        Ok(packet)
    }
}
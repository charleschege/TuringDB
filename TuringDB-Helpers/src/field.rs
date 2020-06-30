use crate::commands::{from_op, TuringOp};
use anyhow::Result;
use serde::Serialize;

/// ### Handles all queries releated to fields
/// ```rust
///#[derive(Debug, Serialize, Clone)]
///pub struct FieldQuery {
///    db: String,
///    document: String,
///    field: String,
///    payload: Option<T>,
///}
///```
#[derive(Debug, Serialize, Clone)]
pub struct FieldQuery<T> {
    db: String,
    document: String,
    field: String,
    payload: Option<T>,
}

impl<T> FieldQuery<T>
where
    T: serde::Serialize,
{
    /// ### Initialize a new empty field
    /// #### Usage
    /// ```rust
    /// use crate::FieldQuery;
    ///
    /// FieldQuery::new().await
    /// ```
    pub async fn new() -> Self {
        Self {
            db: Default::default(),
            document: Default::default(),
            field: Default::default(),
            payload: Default::default(),
        }
    }
    /// ### Add a database name
    /// #### Usage
    /// ```rust
    /// use crate::FieldQuery;
    ///
    /// let mut foo = FieldQuery::new().await;
    /// foo.db("db_name").await;
    /// ```
    pub async fn db(&mut self, name: &str) -> &Self {
        self.db = name.into();

        self
    }
    /// ### Add a document name
    /// #### Usage
    /// ```rust
    /// use crate::FieldQuery;
    ///
    /// let mut foo = FieldQuery::new().await;
    /// foo
    ///   .db("db_name").await
    ///   .document("document_name").await;
    /// ```
    pub async fn document(&mut self, name: &str) -> &Self {
        self.document = name.into();

        self
    }
    /// ### Add a field name
    /// #### Usage
    /// ```rust
    /// use crate::FieldQuery;
    ///
    /// let mut foo = FieldQuery::new().await;
    /// foo
    ///   .db("db_name").await
    ///   .document("document_name").await
    ///   .field("field_name").await;
    /// ```
    pub async fn field(&mut self, name: &str) -> &Self {
        self.field = name.into();

        self
    }
    /// ### Add a payload of bytes
    /// This takes a generic value and convertes it into bytes using bincode
    /// #### Usage
    /// ```rust
    /// use crate::FieldQuery;
    ///
    /// let mut foo = FieldQuery::new().await;
    /// foo
    ///   .db("db_name").await
    ///   .document("document_name").await
    ///   .field("field_name").await
    ///   .payload("my_data_converted_into_bytes".as_bytes()).await;
    /// ```
    pub async fn payload(&mut self, value: T) -> &Self {
        self.payload = Some(value);

        self
    }
    /// ### Inserts a `key/value` to a document in a database
    /// #### Usage
    /// ```rust
    /// use crate::FieldQuery;
    ///
    /// let mut foo = FieldQuery::new().await;
    /// foo
    ///   .db("db_name").await
    ///   .document("document_name").await
    ///   .field("field_name").await
    ///   .payload("my_data_converted_into_bytes".as_bytes()).await
    ///   .set().await
    /// ```
    pub async fn set(&self) -> Result<Vec<u8>> {
        let mut packet = from_op(&TuringOp::FieldInsert).await.to_vec();
        packet.extend_from_slice(self.db.as_bytes());

        let data = bincode::serialize::<Self>(self)?;
        packet.extend_from_slice(&data);

        Ok(packet)
    }
    /// ### Gets a `value` to a document in a database by `key`
    /// #### Usage
    /// ```rust
    /// use crate::FieldQuery;
    ///
    /// let mut foo = FieldQuery::new().await;
    /// foo
    ///   .db("db_name").await
    ///   .document("document_name").await
    ///   .field("field_name").await
    ///   .get().await;
    /// ```
    pub async fn get(&self) -> Result<Vec<u8>> {
        let mut packet = from_op(&TuringOp::FieldGet).await.to_vec();
        packet.extend_from_slice(self.db.as_bytes());

        let data = bincode::serialize::<Self>(self)?;
        packet.extend_from_slice(&data);

        Ok(packet)
    }
    /// ### List all the `keys` in a document
    /// #### Usage
    /// ```rust
    /// use crate::FieldQuery;
    ///
    /// let mut foo = FieldQuery::new().await;
    /// foo
    ///   .db("db_name").await
    ///   .document("document_name").await
    ///   .list().await;
    /// ```
    pub async fn list(&self) -> Result<Vec<u8>> {
        let mut packet = from_op(&TuringOp::FieldList).await.to_vec();
        packet.extend_from_slice(self.db.as_bytes());

        let data = bincode::serialize::<Self>(self)?;
        packet.extend_from_slice(&data);

        Ok(packet)
    }
    /// ### Removes a `value` from a document in a database by `key`
    /// #### Usage
    /// ```rust
    /// use crate::FieldQuery;
    ///
    /// let mut foo = FieldQuery::new().await;
    /// foo
    ///   .db("db_name").await
    ///   .document("document_name").await
    ///   .field("field_name").await
    ///   .remove().await;
    /// ```
    pub async fn remove(&self) -> Result<Vec<u8>> {
        let mut packet = from_op(&TuringOp::FieldRemove).await.to_vec();
        packet.extend_from_slice(self.db.as_bytes());

        let data = bincode::serialize::<Self>(self)?;
        packet.extend_from_slice(&data);

        Ok(packet)
    }
    /// ### Modifies a `value` in a document in a database by its `key`
    /// #### Usage
    /// ```rust
    /// use crate::FieldQuery;
    ///
    /// let mut foo = FieldQuery::new().await;
    /// foo
    ///   .db("db_name").await
    ///   .document("document_name").await
    ///   .field("field_name").await
    ///   .payload("my_data_converted_into_bytes".as_bytes()).await
    ///   .modify().await
    /// ```
    pub async fn modify(&self) -> Result<Vec<u8>> {
        let mut packet = from_op(&TuringOp::FieldModify).await.to_vec();
        packet.extend_from_slice(self.db.as_bytes());

        let data = bincode::serialize::<Self>(self)?;
        packet.extend_from_slice(&data);

        Ok(packet)
    }
}

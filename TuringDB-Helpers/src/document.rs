use serde::Serialize;
use anyhow::Result;
use crate::commands::{from_op, TuringOp};

/// ### Handles all queries releated to fields
/// ```rust
/// #[derive(Debug, Serialize, Clone)]
/// pub struct DocumentQuery {
///     db: String,
///     document: Option<String>,
/// }
/// ```
#[derive(Debug, Serialize, Clone)]
pub struct DocumentQuery {
    db: String,
    document: Option<String>,
}

impl DocumentQuery {
    /// ### Initialize a new empty document
    /// #### Usage
    /// ```rust
    /// use crate::DocumentQuery;
    ///
    /// Document::new().await
    /// ```
    pub async fn new() -> Self {
        Self {
            db: Default::default(),
            document: Default::default(),
        }
    }
    /// ### Add a database name
    /// #### Usage
    /// ```rust
    /// use crate::DocumentQuery;
    ///
    /// let mut foo = DocumentQuery::new().await;
    /// foo.db("db_name").await;
    /// ```
    pub async fn db(&mut self, name: &str) -> &Self {
        self.db = name.into();

        self
    }
    /// ### Add a document name
    /// #### Usage
    /// ```rust
    /// use crate::DocumentQuery;
    ///
    /// let mut foo = DocumentQuery::new().await;
    /// foo
    ///   .db("db_name").await
    ///   .document("document_name").await;
    /// ```
    pub async fn document(&mut self, name: &str) -> &Self {
        self.document = Some(name.into());

        self
    }
    /// ### Creates a new document in a database
    /// #### Usage
    /// ```rust
    /// use crate::DocumentQuery;
    ///
    /// let mut foo = DocumentQuery::new().await;
    /// foo
    ///   .db("db_name").await
    ///   .document("document_name").await
    ///   .create().await
    /// ```
    pub async fn create(&self) -> Result<Vec<u8>> {
        let mut packet = from_op(&TuringOp::DocumentCreate).await.to_vec();
        packet.extend_from_slice(self.db.as_bytes());
        
        let data = bincode::serialize::<Self>(self)?;
        packet.extend_from_slice(&data);

        Ok(packet)
    }
    /// ### List all documents in a database
    /// #### Usage
    /// ```rust
    /// use crate::DocumentQuery;
    ///
    /// let mut foo = DocumentQuery::new().await;
    /// foo
    ///   .db("db_name").await
    ///   .list().await
    /// ```
    pub async fn list(&self) -> Result<Vec<u8>> {
        let mut packet = from_op(&TuringOp::DocumentList).await.to_vec();
        packet.extend_from_slice(self.db.as_bytes());

        let data = bincode::serialize::<Self>(self)?;
        packet.extend_from_slice(&data);

        Ok(packet)
    }
    /// ### Creates a new document in a database
    /// #### Usage
    /// ```rust
    /// use crate::DocumentQuery;
    ///
    /// let mut foo = DocumentQuery::new().await;
    /// foo
    ///   .db("db_name").await
    ///   .document("document_name").await
    ///   .drop().await
    /// ```
    pub async fn drop(&self) -> Result<Vec<u8>> {
        let mut packet = from_op(&TuringOp::DocumentDrop).await.to_vec();
        packet.extend_from_slice(self.db.as_bytes());
        
        let data = bincode::serialize::<Self>(self)?;
        packet.extend_from_slice(&data);

        Ok(packet)
    }
}
use crate::commands::{from_op, TuringOp};
use anyhow::Result;
use serde::Serialize;

/// ### Handles all queries releated to fields
/// ```rust
/// #[derive(Debug, Serialize, Clone)]
/// pub struct DocumentQuery {
///     db: String,
///     document: Option<String>,
/// }
/// ```
#[derive(Debug, Serialize, Clone, Default)]
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
    /// Document::new()
    /// ```
    pub fn new() -> Self {
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
    /// let mut foo = DocumentQuery::new();
    /// foo.db("db_name");
    /// ```
    pub fn db(&mut self, name: &str) -> &Self {
        self.db = name.into();

        self
    }
    /// ### Add a document name
    /// #### Usage
    /// ```rust
    /// use crate::DocumentQuery;
    ///
    /// let mut foo = DocumentQuery::new();
    /// foo
    ///   .db("db_name")
    ///   .document("document_name");
    /// ```
    pub fn document(&mut self, name: &str) -> &Self {
        self.document = Some(name.into());

        self
    }
    /// ### Creates a new document in a database
    /// #### Usage
    /// ```rust
    /// use crate::DocumentQuery;
    ///
    /// let mut foo = DocumentQuery::new();
    /// foo
    ///   .db("db_name")
    ///   .document("document_name")
    ///   .create()
    /// ```
    pub fn create(&self) -> Result<Vec<u8>> {
        let mut packet = from_op(&TuringOp::DocumentCreate).to_vec();
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
    /// let mut foo = DocumentQuery::new();
    /// foo
    ///   .db("db_name")
    ///   .list()
    /// ```
    pub fn list(&self) -> Result<Vec<u8>> {
        let mut packet = from_op(&TuringOp::DocumentList).to_vec();
        packet.extend_from_slice(self.db.as_bytes());

        let data = bincode::serialize::<Self>(self)?;
        packet.extend_from_slice(&data);

        Ok(packet)
    }
    /// ### Drops document in a database
    /// #### Usage
    /// ```rust
    /// use crate::DocumentQuery;
    ///
    /// let mut foo = DocumentQuery::new();
    /// foo
    ///   .db("db_name")
    ///   .document("document_name")
    ///   .drop()
    /// ```
    pub fn drop(&self) -> Result<Vec<u8>> {
        let mut packet = from_op(&TuringOp::DocumentDrop).to_vec();
        packet.extend_from_slice(self.db.as_bytes());

        let data = bincode::serialize::<Self>(self)?;
        packet.extend_from_slice(&data);

        Ok(packet)
    }
}

use crate::commands::{from_op, TuringOp};

/// ### Handles all queries releated to fields
/// ```rust
/// #[derive(Debug, Clone)]
/// pub struct DbQuery {
///     db: String,
/// }
/// ```
#[derive(Debug, Clone)]
pub struct DbQuery {
    db: String,
}

impl<'tp> DbQuery {
    /// ### Initialize a new empty database
    /// #### Usage
    /// ```rust
    /// use crate::DatabaseQuery;
    ///
    /// Database::new().await
    /// ```
    pub async fn new() -> Self {
        Self {
            db: Default::default(),
        }
    }
    /// ### Add a database name
    /// #### Usage
    /// ```rust
    /// use crate::DatabaseQuery;
    ///
    /// let mut foo = Database::new().await;
    /// foo.db("db_name").await;
    /// ```
    pub async fn db(&mut self, name: &str) -> &Self {
        self.db = name.into();

        self
    }
    /// ### Creates a new a database in a repo
    /// #### Usage
    /// ```rust
    /// use crate::DatabaseQuery;
    ///
    /// let mut foo = DatabaseQuery::new().await;
    /// foo
    ///   .db("db_name").await
    ///   .create().await
    /// ```
    pub async fn create(&self) -> Vec<u8> {
        let mut packet = from_op(&TuringOp::DbCreate).await.to_vec();
        packet.extend_from_slice(self.db.as_bytes());

        packet
    }
    /// ### Creates a new a database in a repo
    /// #### Usage
    /// ```rust
    /// use crate::DatabaseQuery;
    ///
    /// let mut foo = DatabaseQuery::new().await;
    /// foo
    ///   .db("db_name").await
    ///   .drop().await
    /// ```
    pub async fn drop(&self) -> Vec<u8> {
        let mut packet = from_op(&TuringOp::DbDrop).await.to_vec();
        packet.extend_from_slice(self.db.as_bytes());

        packet
    }
    /// ### Creates a new a database in a repo
    /// #### Usage
    /// ```rust
    /// use crate::DatabaseQuery;
    ///
    /// let mut foo = DatabaseQuery::new().await;
    /// foo
    ///   .db("db_name").await
    ///   .list().await
    /// ```
    pub async fn list(&self) -> &'tp [u8] {
        from_op(&TuringOp::DbList).await
    }
}

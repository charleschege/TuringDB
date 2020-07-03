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
    /// Database::new()
    /// ```
    pub fn new() -> Self {
        Self {
            db: Default::default(),
        }
    }
    /// ### Add a database name
    /// #### Usage
    /// ```rust
    /// use crate::DatabaseQuery;
    ///
    /// let mut foo = Database::new();
    /// foo.db("db_name");
    /// ```
    pub fn db(&mut self, name: &str) -> &Self {
        self.db = name.into();

        self
    }
    /// ### Creates a new a database in a repo
    /// #### Usage
    /// ```rust
    /// use crate::DatabaseQuery;
    ///
    /// let mut foo = DatabaseQuery::new();
    /// foo
    ///   .db("db_name")
    ///   .create()
    /// ```
    pub fn create(&self) -> Vec<u8> {
        let mut packet = from_op(&TuringOp::DbCreate).to_vec();
        packet.extend_from_slice(self.db.as_bytes());

        packet
    }
    /// ### Creates a new a database in a repo
    /// #### Usage
    /// ```rust
    /// use crate::DatabaseQuery;
    ///
    /// let mut foo = DatabaseQuery::new();
    /// foo
    ///   .db("db_name").await
    ///   .drop().await
    /// ```
    pub fn drop(&self) -> Vec<u8> {
        let mut packet = from_op(&TuringOp::DbDrop).to_vec();
        packet.extend_from_slice(self.db.as_bytes());

        packet
    }
    /// ### List all databases in a repo
    /// #### Usage
    /// ```rust
    /// use crate::DatabaseQuery;
    ///
    /// let mut foo = DatabaseQuery::new();
    /// foo.list()
    /// ```
    pub fn list(&self) -> &'tp [u8] {
        from_op(&TuringOp::DbList)
    }
}

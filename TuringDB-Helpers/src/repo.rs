use crate::commands::{from_op, TuringOp};

/// #### This struct handles all `repo` related queries like `dropping a repo or creating one`
/// ```rust
/// #[derive(Debug, Clone)]
/// pub struct RepoQuery;
/// ```
#[derive(Debug, Clone)]
pub struct RepoQuery;

impl<'tp> RepoQuery {
    /// ### Create a repository
    /// #### Usage
    /// ```rust
    /// use crate::repo::RepoQuery;
    ///
    /// RepoQuery::create()
    /// ```
    pub fn create() -> &'tp [u8] {
        from_op(&TuringOp::RepoCreate)
    }
    /// ### Drop a repository
    /// #### Usage
    /// ```rust
    /// use crate::repo::RepoQuery;
    ///
    /// RepoQuery::drop()
    /// ```
    pub fn drop() -> &'tp [u8] {
        from_op(&TuringOp::RepoDrop)
    }
}

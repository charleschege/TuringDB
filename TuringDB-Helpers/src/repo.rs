use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepoQuery;

impl RepoQuery {
    pub async fn new() -> Self {
       RepoQuery
    }
    pub async fn own(&self) -> Self {

        self.to_owned()
    }
}
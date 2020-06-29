use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbQuery {
    db: String,
    document: Option<String>,
}

impl DbQuery {
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
}
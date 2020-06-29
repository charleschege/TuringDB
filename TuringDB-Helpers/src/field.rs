use serde::{Serialize, Deserialize};
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
}
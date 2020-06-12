use serde::{Deserialize, Serialize};
use custom_codes::DbOps;
use crate::TuringCommands;
use anyhow::Result;
use async_std::{
    net::{TcpStream,},
    io::prelude::*,
};

const ADDRESS: &'static str = "127.0.0.1:43434";
const BUFFER_CAPACITY: usize = 64 * 1024; //16Kb

#[allow(dead_code)]
const BUFFER_DATA_CAPACITY: usize = 1024 * 1024 * 16; //TODO Db cannot hold data more than 16MB in size

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq)]
pub struct FieldWithData {
    pub db: String,
    pub document: String,
    pub field: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq)]
pub struct FieldNoData {
    pub db: String,
    pub document: String,
    pub field: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq)]
pub struct DocumentOnly {
    pub db: String,
    pub document: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq)]
pub struct RepoBuilder;

impl RepoBuilder {
    pub async fn create() -> Result<DbOps> {
        executor(TuringCommands::RepoCreate).await
    }

    pub async fn drop() -> Result<DbOps> {
        executor(TuringCommands::RepoDrop).await
    }

    pub async fn read() -> Result<DbOps> {
        executor(TuringCommands::DbList).await
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Default)]
pub struct DbBuilder {
    db: String,
}

impl DbBuilder {
    pub async fn new() -> Self {
        Self { ..Default::default() }
    }

    pub async fn db(&mut self, db_name: &str) -> &Self {
        self.db = db_name.into();

        self
    }
    
    pub async fn insert(&self) -> Result<DbOps> {
        executor(TuringCommands::DbCreate(self.db.clone())).await
    }
    
    pub async fn get(&self) -> Result<DbOps> {
        executor(TuringCommands::DbRead(self.db.clone())).await
    }
    
    pub async fn drop(&self) -> Result<DbOps> {
        executor(TuringCommands::DbDrop(self.db.clone())).await
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Default)]
pub struct DocumentBuilder {
    db: String,
    document: String,
}

impl DocumentBuilder {
    pub async fn new() -> Self {
        Self { ..Default::default() }
    }

    pub async fn db(&mut self, db_name: &str) -> &mut Self {
        self.db = db_name.into();

        self
    }

    pub async fn document(&mut self, document_name: &str) -> &mut Self {
        self.document = document_name.into();

        self
    }
    
    pub async fn insert(&self) -> Result<DbOps> {
        executor(TuringCommands::DocumentCreate(DocumentOnly{
            db: self.db.clone(),
            document: self.document.clone(),
        })).await
    }
    
    pub async fn get(&self) -> Result<DbOps> {
        executor(TuringCommands::DocumentRead(DocumentOnly{
            db: self.db.clone(),
            document: self.document.clone(),
        })).await
    }
    
    pub async fn drop(&self) -> Result<DbOps> {
        executor(TuringCommands::DocumentDrop(DocumentOnly{
            db: self.db.clone(),
            document: self.document.clone(),
        })).await
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq)]
pub struct FieldBuilder {
    db: String,
    document: String,
    field: String,
    data: Vec<u8>,
}

impl FieldBuilder {
    pub async fn new() -> Self {
        Self {
            db: String::default(),
            document: String::default(),
            field: String::default(),
            data: Vec::default(),
        }
    }

    pub async fn db(&mut self, db_name: &str) -> &mut Self {
        self.db = db_name.into();

        self
    }

    pub async fn document(&mut self, document_name: &str) -> &mut Self {
        self.document = document_name.into();

        self
    }

    pub async fn field(&mut self, field_name: &str) -> &mut Self {
        self.field = field_name.into();

        self
    }

    pub async fn insert<'de, T>(&self, values: T) -> Result<DbOps>
        where 
            T: std::fmt::Debug + Sync + Send + serde::Serialize + serde::Deserialize<'de>,
    {
        executor(TuringCommands::FieldInsert(FieldWithData{
            db: self.db.clone(),
            document: self.document.clone(),
            field: self.field.clone(),
            data: bincode::serialize::<T>(&values)?,
        })).await
    }

    pub async fn get(&self) -> Result<DbOps> {
        executor(TuringCommands::FieldRead(FieldNoData{
            db: self.db.clone(),
            document: self.document.clone(),
            field: self.field.clone(),
        })).await
    }

    pub async fn update<'de, T>(&self, values: T) -> Result<DbOps>
        where 
            T: std::fmt::Debug + Sync + Send + serde::Serialize + serde::Deserialize<'de>,
    {
        executor(TuringCommands::FieldModify(FieldWithData{
            db: self.db.clone(),
            document: self.document.clone(),
            field: self.field.clone(),
            data: bincode::serialize::<T>(&values)?,
        })).await
    }

    pub async fn drop(&self) -> Result<DbOps> {
        executor(TuringCommands::FieldRemove(FieldNoData{
            db: self.db.clone(),
            document: self.document.clone(),
            field: self.field.clone(),
        })).await
    }
}

async fn executor<'de, T>(values: T) -> Result<DbOps> 
    where 
        T: std::fmt::Debug + Sync + Send + serde::Serialize + serde::Deserialize<'de>,
{
    let mut stream = TcpStream::connect(ADDRESS).await?;
    let to_bytes = bincode::serialize::<T>(&values)?;
    
    stream.write(&to_bytes.len().to_le_bytes()).await?;
    stream.write(&to_bytes).await?;
    stream.flush().await?;

    let mut header: [u8; 8] = [0; 8];

    let mut buffer = [0; BUFFER_CAPACITY];
    let mut container_buffer: Vec<u8> = Vec::new();
    let mut bytes_read: usize;

    stream.read(&mut header).await?;

    //Get the length of the data first
    let stream_byte_size = usize::from_le_bytes(header);
    let mut current_buffer_size = 0_usize;
    
    loop {
        bytes_read = stream.read(&mut buffer).await?;

        // Add the new buffer length to the current buffer size
        current_buffer_size += buffer[..bytes_read].len();

        if current_buffer_size == stream_byte_size {
            // Ensure that the data is appended before being deserialized by bincode
            container_buffer.append(&mut buffer[..bytes_read].to_owned());
            let data = bincode::deserialize::<DbOps>(&container_buffer)?;
            return Ok(data)
        }
        // Append data to buffer
        container_buffer.append(&mut buffer[..bytes_read].to_owned());
    }

    // TODO Ensure client does not immediately terminate
}
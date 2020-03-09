use async_std::{fs::OpenOptions, io::prelude::*, path::PathBuf};
use custom_codes::FileOps;
use tai64::TAI64N;
use anyhow::Result;

#[derive(Debug)]
pub enum Operation {
    DbInsert,
    DbLookUp,
    DbUpdate,
    DbDelete,
    TableInsert,
    TableLookUp,
    TableUpdate,
    TableDelete,
    Unspecified,
}

impl Default for Operation {
    fn default() -> Self {
        Self::Unspecified
    }
}

#[derive(Debug)]
pub struct ErrorLogger<T> {
    error_type: Result<T>,
    time: TAI64N,
    operation: Operation,
}

impl<T> ErrorLogger<T> where T: std::fmt::Debug {
    pub async fn new(data: Result<T>) -> Self {
        Self {
            error_type: data,
            time: TAI64N::now(),
            operation: Default::default(),
        }
    }

    pub async fn error_type(mut self, value: Result<T>) -> Self {
        self.error_type = value;

        self
    }

    pub async fn op(mut self, value: Operation) -> Self {
        self.operation = value;

        self
    }

    pub async fn log(self) -> Result<FileOps> {
        let mut db_path = PathBuf::new();
        db_path.push("TuringFeedsDB");
        db_path.push("TuringFeeds.log");

        let mut file = OpenOptions::new()
            .create(true)
            .write(false)
            .append(true)
            .open(db_path)
            .await?;

        writeln!(file, "{:?}", self).await?;
        
        Ok(FileOps::AppendTrue)
    }
}

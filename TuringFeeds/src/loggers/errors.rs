use async_std::{fs::OpenOptions, io::prelude::*, path::PathBuf};
use custom_codes::FileOps;
use tai64::TAI64N;
use turingfeeds_helpers::TuringFeedsError;

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
pub struct ErrorLogger {
    kind: TuringFeedsError,
    time: TAI64N,
    operation: Operation,
}

impl ErrorLogger {
    pub async fn new() -> Self {
        Self {
            kind: Default::default(),
            time: TAI64N::now(),
            operation: Default::default(),
        }
    }

    pub async fn kind(mut self, value: TuringFeedsError) -> Self {
        self.kind = value;

        self
    }

    pub async fn op(mut self, value: Operation) -> Self {
        self.operation = value;

        self
    }

    pub async fn log(self) -> Result<FileOps, TuringFeedsError> {
        let mut db_path = PathBuf::new();
        db_path.push("TuringFeedsDB");
        db_path.push("TuringFeeds.log");

        let mut file = OpenOptions::new()
            .create(true)
            .write(false)
            .append(true)
            .open(db_path)
            .await?;

        match writeln!(file, "{:?}", self).await {
            Ok(_) => Ok(FileOps::AppendTrue),
            Err(error) => Err(TuringFeedsError::IoError(error)),
        }
    }
}

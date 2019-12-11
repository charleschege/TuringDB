use tai64::TAI64N;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use crate::TuringFeedsError;
use custom_codes::FileOps;

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
pub struct ErrorLogger{
    kind: TuringFeedsError,
    time: TAI64N,
    operation: Operation
}

impl ErrorLogger {
    pub fn new() -> Self {
        Self {
            kind: Default::default(),
            time: TAI64N::now(),
            operation: Default::default(),
        }
    }

    pub fn kind(mut self, value: TuringFeedsError) -> Self {
        self.kind = value;

        self
    }

    pub fn op(mut self, value: Operation) -> Self {
        self.operation = value;

        self
    }

    pub fn log(self) -> Result<FileOps, TuringFeedsError> {
        let mut db_path = PathBuf::new();
		db_path.push("TuringFeedsDB");
		db_path.push("TuringFeeds.log");

        let mut file = OpenOptions::new()
            .create(true)
            .write(false)
            .append(true)
            .open(db_path)?;    

		match writeln!(file, "{:?}", self)
			{
				Ok(_) => Ok(FileOps::AppendTrue),
				Err(error) => Err(TuringFeedsError::IoError(error)),
			}
    }

    
}


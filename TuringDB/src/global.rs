use async_lock::Mutex;
use camino::{Utf8Path, Utf8PathBuf};
use sled::IVec;
use std::io::ErrorKind;

use crate::TuringDB;

const REPO_NAME: &str = "TuringDB-Repo";

pub type TuringResult<T> = Result<T, TuringDbError>;
pub type Document = sled::Db;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TuringDbError {
    UserHomeDirMissing,
    UserHomeDirIsInvalidUtf8Path,
    PathReadIsNotUtf8Path,
    DbNameMissing,
    DbNotFound,
    DocumentNotFound,
    KeyAlreadyExists,
    InvalidPathUnicodeName,
    NotFound,
    PermissionDenied,
    ConnectionRefused,
    ConnectionReset,
    ConnectionAborted,
    NotConnected,
    AddrInUse,
    AddrNotAvailable,
    BrokenPipe,
    AlreadyExists,
    WouldBlock,
    InvalidInput,
    InvalidData,
    TimedOut,
    WriteZero,
    Interrupted,
    Other(String),
    UnexpectedEof,
    DocumentNoLongerExists,
    SystemViolation(String),
    Bug(String),
    DocumentCorrupted { at: Option<sled::DiskPtr>, bt: () },
}

impl From<std::io::Error> for TuringDbError {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            ErrorKind::NotFound => TuringDbError::NotFound,
            ErrorKind::PermissionDenied => TuringDbError::PermissionDenied,
            ErrorKind::ConnectionRefused => TuringDbError::ConnectionRefused,
            ErrorKind::ConnectionReset => TuringDbError::ConnectionReset,
            ErrorKind::ConnectionAborted => TuringDbError::ConnectionAborted,
            ErrorKind::NotConnected => TuringDbError::NotConnected,
            ErrorKind::AddrInUse => TuringDbError::AddrInUse,
            ErrorKind::AddrNotAvailable => TuringDbError::AddrNotAvailable,
            ErrorKind::BrokenPipe => TuringDbError::BrokenPipe,
            ErrorKind::AlreadyExists => TuringDbError::AlreadyExists,
            ErrorKind::WouldBlock => TuringDbError::WouldBlock,
            ErrorKind::InvalidInput => TuringDbError::InvalidInput,
            ErrorKind::InvalidData => TuringDbError::InvalidData,
            ErrorKind::TimedOut => TuringDbError::TimedOut,
            ErrorKind::WriteZero => TuringDbError::WriteZero,
            ErrorKind::Interrupted => TuringDbError::Interrupted,
            ErrorKind::Other => TuringDbError::Other(error.to_string()),
            ErrorKind::UnexpectedEof => TuringDbError::UnexpectedEof,
            _ => {
                let mut error_from_new_rust_release = String::new();
                error_from_new_rust_release.push_str(&format!("{:?}", error.kind()));

                TuringDbError::Bug(error_from_new_rust_release)
            }
        }
    }
}

impl From<sled::Error> for TuringDbError {
    fn from(error: sled::Error) -> Self {
        match error {
            sled::Error::CollectionNotFound(_) => TuringDbError::DocumentNoLongerExists,
            sled::Error::Unsupported(value) => TuringDbError::SystemViolation(value),
            sled::Error::ReportableBug(value) => TuringDbError::Bug(value),
            sled::Error::Io(value) => value.into(),
            sled::Error::Corruption { at, bt } => TuringDbError::DocumentCorrupted { at, bt },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpsOutcome {
    /// A temporary value for testing
    OpsOutcomePlaceholder,
    RepoCreated,
    RepoInitialized,
    RepoEmpty,
    DbCreated,
    DbDropped,
    DbList(Vec<Utf8PathBuf>),
    DbEmpty,
    DocumentList(Vec<Utf8PathBuf>),
    DocumentCreated,
    DocumentDropped,
    FieldInserted,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct RepoPath;

impl RepoPath {
    pub(crate) async fn access_dir() -> Result<Utf8PathBuf, TuringDbError> {
        match directories::UserDirs::new() {
            None => Err(TuringDbError::UserHomeDirMissing),
            Some(user_dir) => {
                let home_dir = match user_dir.home_dir().to_str() {
                    None => return Err(TuringDbError::PathReadIsNotUtf8Path),
                    Some(dir) => dir,
                };
                let mut repo_path = Utf8PathBuf::new();
                repo_path.push(home_dir);
                repo_path.push(REPO_NAME);

                Ok(repo_path)
            }
        }
    }
}

pub type DBName = Utf8PathBuf;
pub type RepoName = Utf8PathBuf;
pub type DocumentName = Utf8PathBuf;
pub type FieldKey = DataType;
pub type FieldValue = DataType;

pub struct TuringDBOps(DBName);

impl Default for TuringDBOps {
    fn default() -> Self {
        Self(Utf8PathBuf::default())
    }
}

impl TuringDBOps {
    pub fn set_db_name(mut self, db_name: &str) -> Self {
        self.0 = Utf8Path::new(&db_name).to_path_buf();

        self
    }

    pub fn get_db_name(&self) -> Utf8PathBuf {
        self.0.to_owned()
    }
}
pub struct TuringDBDocumentOps {
    db_name: DBName,
    document_name: DocumentName,
}

impl Default for TuringDBDocumentOps {
    fn default() -> Self {
        Self {
            db_name: DBName::default(),
            document_name: DocumentName::default(),
        }
    }
}

impl TuringDBDocumentOps {
    pub fn set_db_name(mut self, db_name: &str) -> Self {
        self.db_name = Utf8Path::new(&db_name).to_path_buf();

        self
    }

    pub fn set_document_name(mut self, document_name: &str) -> Self {
        self.document_name = Utf8Path::new(&document_name).to_path_buf();

        self
    }

    pub fn get_db_name(&self) -> Utf8PathBuf {
        self.db_name.to_owned()
    }

    pub fn get_document_name(&self) -> Utf8PathBuf {
        self.document_name.to_owned()
    }
}

pub struct TuringDBFieldOps {
    db_name: DBName,
    document_name: DocumentName,
    field_name: FieldKey,
    field_value: FieldValue,
}

impl TuringDBFieldOps {
    pub fn db(mut self, db_name: &str) -> Self {
        self.db_name = Utf8Path::new(&db_name).to_path_buf();

        self
    }

    pub fn document(mut self, document_name: &str) -> Self {
        self.document_name = Utf8Path::new(&document_name).to_path_buf();

        self
    }

    pub fn get_db_name(&self) -> Utf8PathBuf {
        self.db_name.to_owned()
    }

    pub fn get_document_name(&self) -> Utf8PathBuf {
        self.document_name.to_owned()
    }

    pub fn get_key(&self) -> FieldKey {
        self.field_name
    }

    pub fn get_value(&self) -> FieldValue {
        self.field_value
    }
}

// TODO. Add these as features support but Borsh type is default
/*
pub struct TuringDBFieldOps {
    db_name: DBName,
    document_name: DocumentName,
    field_name: FieldName,
    field_value: TuringDBValue,
}

pub struct TuringDBValue {
    data: String,
    ser_der: SerDerType,
}

enum SerDerType {
    Borsh,
    Bincode,
    JSON,
    CBOR,
}
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DataType {
    Boolean = 0x00,
    U8 = 0x01,
    I8 = 0x02,
    U16 = 0x03,
    I16 = 0x04,
    U32 = 0x05,
    I32 = 0x06,
    U64 = 0x07,
    I64 = 0x08,
    U128 = 0x09,
    I128 = 0x10,
    F32 = 0x11,
    F64 = 0x12,
    STRING = 0x13,
    ARRAY = 0x14,
    UTC = 0x15,
    TAI64 = 0x16,
    TAI64N = 0x17,
    TAI64NA = 0x18,
    RANGE = 0x19,
    TIMESPEC = 0x20,
    OPTION = 0x21,
    BLAKE3 = 0x22,
    BLAKE3HMAC = 0x23,
    SHA3 = 0x24,
    SHA3HMAC = 0x25,
    BORSCH = 0x26,
    GEO = 0x27,
    BINARY = 0x28,
    CHACHA8 = 0x29,
    CHACHA12 = 0x30,
    CHACHA20 = 0x31,
    CHACHAPOLY1305 = 0x32,
    XCHACHABLAKE3SIV = 0x33,
    AES256GCM = 0x34,
}

const TRUE: u8 = 1;
const FALSE: u8 = 1;

pub struct TDBCell {
    data_type: DataType,
    data: Vec<u8>,
}

impl TDBCell {
    pub fn data_type(&mut self, value: DataType) -> &mut Self {
        self.data_type = value;

        self
    }

    pub fn data(&mut self, value: &[u8]) -> &mut Self {
        self.data = value.to_owned();

        self
    }

    pub fn to_ivec(&self) -> IVec {
        let mut data = Vec::default();
        data.push(self.data_type as u8);
        data.extend_from_slice(&self.data);

        IVec::from(data)
    }
}

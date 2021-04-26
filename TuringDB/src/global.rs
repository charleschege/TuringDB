use async_lock::Mutex;
use camino::{Utf8Path, Utf8PathBuf};
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
pub type FieldName = Utf8PathBuf;

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
    field_name: FieldName,
}

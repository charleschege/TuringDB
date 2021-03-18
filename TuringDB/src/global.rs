use camino::{Utf8Path, Utf8PathBuf};
use std::io::ErrorKind;

const REPO_NAME: &str = "TuringDB-Repo";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TuringDbError {
    UserHomeDirMissing,
    UserHomeDirIsInvalidUtf8Path,
    PathReadIsNotUtf8Path,
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
    Other,
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
            ErrorKind::Other => TuringDbError::Other,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpsOutcome {
    /// A temporary value for testing
    OpsOutcomePlaceholder,
    RepoCreated,
    RepoInitialized,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct RepoUtf8Path;

impl RepoUtf8Path {
    pub(crate) async fn access_dir() -> Result<Utf8PathBuf, TuringDbError> {
        match directories::UserDirs::new() {
            None => Err(TuringDbError::UserHomeDirMissing),
            Some(user_dir) => {
                let safe_user_dir = match Utf8Path::from_path(user_dir.home_dir()) {
                    None => return Err(TuringDbError::UserHomeDirIsInvalidUtf8Path),
                    Some(dir) => dir,
                };
                let mut repo_path = Utf8PathBuf::new();
                repo_path.push(safe_user_dir);
                repo_path.push(REPO_NAME);

                Ok(repo_path)
            }
        }
    }
}

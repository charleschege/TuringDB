#[derive(Debug)]
pub enum TuringFeedsError {
    IoError(async_std::io::Error),
    WalkDirError(walkdir::Error),
    RonSerError(ron::ser::Error),
    RonDeError(ron::de::Error),
    BincodeError(bincode::Error),
    Unspecified,
}

impl Default for TuringFeedsError {
    fn default() -> Self {
        Self::Unspecified
    }
}

impl From<async_std::io::Error> for TuringFeedsError {
    fn from(error: async_std::io::Error) -> Self {
        TuringFeedsError::IoError(error)
    }
}

impl std::fmt::Display for TuringFeedsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TuringFeedsError::IoError(error)",)
    }
}

impl From<walkdir::Error> for TuringFeedsError {
    fn from(error: walkdir::Error) -> Self {
        TuringFeedsError::WalkDirError(error)
    }
}

impl From<ron::ser::Error> for TuringFeedsError {
    fn from(error: ron::ser::Error) -> Self {
        TuringFeedsError::RonSerError(error)
    }
}

impl From<ron::de::Error> for TuringFeedsError {
    fn from(error: ron::de::Error) -> Self {
        TuringFeedsError::RonDeError(error)
    }
}

impl From<bincode::Error> for TuringFeedsError {
    fn from(error: bincode::Error) -> Self {
        TuringFeedsError::BincodeError(error)
    }
}
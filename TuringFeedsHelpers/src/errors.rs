use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub enum TuringFeedsError {
    IoError(async_std::io::Error),
    OsString(std::ffi::OsString),
    RonSerError(ron::ser::Error),
    RonDeError(ron::de::Error),
    BincodeError(bincode::Error),
    BufferDataCapacityFull,
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

impl From<std::ffi::OsString> for TuringFeedsError {
    fn from(error: std::ffi::OsString) -> Self {
        TuringFeedsError::OsString(error)
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
/// A list of all possible errors for easier serializing and deserializing especially when sending down a stream
/// This were created due to difficulties in add serde features to send down the stream
/// Might also help where data is being converted into other formats
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum OperationErrors {
    Io(IoErrors),
    Buffer(BufferErrors),
    Bincode(BincodeErrors),
    Ron(RonErrors),
    Integrity(IntegrityErrors),
    DbOps(custom_codes::DbOps),
    Unspecified,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum BufferErrors {
    CapacityFull,
    BufferEmpty,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum RonErrors {
    RonSerError,
    RonDeError,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum IoErrors {    
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
    UnexpectedEof,
    Other,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum BincodeErrors {
    Io(IoErrors),
    InvalidUtf8Encoding,
    InvalidBoolEncoding(u8),
    InvalidCharEncoding,
    InvalidTagEncoding(usize),
    DeserializeAnyNotSupported,
    SizeLimit,
    SequenceMustHaveLength,
    Custom(String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum IntegrityErrors {
    IntegrityConsistent,
    IntegrityCorrupted,
}
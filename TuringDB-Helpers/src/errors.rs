use serde::{Serialize, Deserialize};

/// A list of all possible errors for easier serializing and deserializing especially when sending down a stream
/// This were created due to difficulties in add serde features to send down the stream
/// Might also help where data is being converted into other formats
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum OpsErrors {
    BufferCapacityExceeded16Mb,
    BufferFull,
    BufferEmpty,
    RonSerError(String),
    RonDeError(String),
    BincodeErrors(String),
    IntegrityConsistent,
    IntegrityCorrupted,
    Io(IoErrors),
    Unspecified,
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

use std::error::Error;

impl std::fmt::Display for OpsErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for OpsErrors {
    fn description(&self) -> &str {
        "TuringFeedsHelpers::OpsErrors"
    }

    fn cause(&self) -> Option<&dyn Error> {
        Some(self)
    }
}
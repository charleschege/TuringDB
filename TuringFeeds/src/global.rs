use std::{iter, path::PathBuf};
use tai64::TAI64N;

use rand::{distributions::Alphanumeric, thread_rng, Rng};

use turingfeeds_helpers::TuringFeedsError;

pub type Result<T> = std::result::Result<T, TuringFeedsError>;

pub type CorruptionGuard = (PathBuf, u64); // (Path, seahash)
pub type Blake2hash = String;

#[derive(Debug, PartialEq, Clone)]
pub struct RandIdentifier;

impl RandIdentifier {
    pub async fn build() -> String {
        let mut rng = thread_rng();

        iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(64)
            .collect::<String>()
            .to_lowercase()
    }
}

#[derive(Debug)]
pub enum AccessRights {
    Table(String),
    Db(String),
}

#[derive(Debug)]
pub enum Role {
    SuperUser,
    Admin,
    SubAdmin,
    User,
}

#[derive(Debug)]
pub enum DbType {
    KeyValueStore,
    RealTimeFeeds,
}

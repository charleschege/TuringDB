#![forbid(unsafe_code)]

use async_std::{
    task,
    fs::{File, OpenOptions},
    net::{TcpListener, TcpStream},
    io::{ErrorKind, prelude::*, stdout},
};

use turingfeeds::{
    TuringFeeds,
    TuringFeedsError,
    Result,
};

/// When using a queue to log data, ensure that the queue work completes before the db shuts down
/// Otherwise, if a user forcefully shuts down the db, abort queue work
/// Use difference to update logs
/// Add access rights and prevent adding rights without a superuser and access key
/// Store access keys and blake2b hashes for databases in zbox
/// Encrypt the database
#[async_std::main]
async fn main() -> Result<()>{
    // Check if database repository exists, if not exit with an error
    match TuringFeeds::new().await.init().await {
        Ok(val) => {
            dbg!(val);
        },
        Err(error) => {
            match error {
                TuringFeedsError::IoError(io_error) => {
                    if io_error.kind() == ErrorKind::NotFound {
                        writeln!(stdout(), "[✘ TURINGFEEDS] \nDatabase Doesn't Exist. Consider Creating One First!").await?;
                    }
                    
                    if io_error.kind() == ErrorKind::PermissionDenied {
                        writeln!(stdout(),"[✘ TURINGFEEDS → PERMISSION DENIED] \nPermission To Access Repository is DENIED! \nCheck That You Have PERMISSION To ACCESS The Repository.").await?
                    }
                    
                    if io_error.kind() == ErrorKind::UnexpectedEof {
                        writeln!(stdout(),"[✘ TURINGFEEDS → CORRUPTED] \nCORRUPTED! Not Read The Whole File.").await?
                    }
                },
                TuringFeedsError::RonDeError(error) => writeln!(stdout(),"[✘ TURINGFEEDS → INITIALIZE ERROR] \nThe metadata file `REPO.log` seems to be corrupted. This file is used to initialize the Database Repository!\nTechnical error: {:?}", error).await?,
                _ =>  writeln!(stdout(),"[TURINGFEEDS] \n{:?}", error).await?
            }
        }
    }

    Ok(())
}
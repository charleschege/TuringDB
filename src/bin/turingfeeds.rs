#![forbid(unsafe_code)]

use async_std::{
    task,
    fs::{File, OpenOptions},
    net::{TcpListener, TcpStream},
    io::{ErrorKind, prelude::*, stdout},
};

use turingfeeds::{
    TuringFeeds,
    TuringFeedsDB,
    TFDocument,
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
    let data = TuringFeeds::new().await;
    dbg!(&data);
    dbg!(data.init().await?);

    /*match TuringFeeds::new().await.init().await {
        Ok(val) => {
            data = val;
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
    }*/
    let document = TFDocument::new().await;
    let db = TuringFeedsDB::new().await;
/*
    for n in 1..6 {
        let table = format!("trial_table{}", n);
        let id = format!("trial_table{}", n);
        document.clone().id(&table).await;
        db.clone().identifier(&id).await
            .memdb_add(document.clone()).await;

        dbg!(&db);

        let end = data.clone().memdb_add(db.clone()).await;

        dbg!(&end);
    }*/

        // TODO Seek the end of the log
    
    //dbg!(&data);
    Ok(())
}
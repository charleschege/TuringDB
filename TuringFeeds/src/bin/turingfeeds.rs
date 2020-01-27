#![forbid(unsafe_code)]

use async_std::{
    net::{TcpListener, TcpStream, SocketAddr},
    task,
    prelude::*,
    io::ErrorKind,    
};
use custom_codes::DbOps;

use turingfeeds::{Result, TuringFeeds, TuringFeedsDB, TFDocument};
use turingfeeds_helpers::{OpsOutcome, TuringFeedsError, DocumentMethods, RepoCommands, PrivilegedTuringCommands, UnprivilegedTuringCommands, SuperUserTuringCommands, OperationErrors, IntegrityErrors, TuringTerminator};

const ADDRESS: &str = "127.0.0.1:43434";
const BUFFER_CAPACITY: usize = 64 * 1024; //16Kb
const BUFFER_DATA_CAPACITY: usize = 1024 * 1024 * 16; // Db cannot hold data more than 16MB in size


use once_cell::sync::OnceCell;
static REPO: OnceCell<TuringFeeds> = OnceCell::new();

// TODO 0. Move RwLock to lock a specific database instead of the whole REPO
// TODO 1. CREATE REPO
// TODO 2. DROP REPO
// TODO 3. CREATE DATABASE
// TODO LIST DATABASES
// TODO 4. DROP DATABASE
// TODO 5. GET DATABASE STATISTICS
// TODO 6. CREATE TABLE
// TODO 7. READ TABLE
// TODO LIST TABLES
// TODO 8. UPDATE TABLE
// TODO 9. DELETE TABLE
// TODO 10. Improve error to the client using the `error_to_client()` function
// TODO 11. Add json conversion for replying to other programming languages
// TODO Add state machines to ensure successful command completion before shutdown like Idle, Processing, Finished try Rc or a counter

#[async_std::main]
async fn main() -> Result<()> {
    // Check if database repository exists, if not exit with an error
    match REPO.set(TuringFeeds::new().await.init().await?) {
        Ok(_) => (),
        Err(error) => { eprintln!("{:?}", error); panic!(); }
    }

    dbg!(&REPO);   
    
    match TcpListener::bind(ADDRESS).await {
        Ok(listener) => {
            println!("Listening on Address: {}", listener.local_addr()?);
            while let Some(stream) = listener.incoming().next().await {
                let stream = stream?;
                task::spawn(async {
                    match handle_client(stream).await {
                        Ok(addr) => {
                            println!("[TERMINATED] ip({}) port({})", addr.ip(), addr.port())
                        },
                        Err(error) => {
                            //--eprintln!("{:?}", errors_printable(error).await);
                        },
                    }
                });
            }
        },
        Err(error) => {
            panic!(error);
        }
    }

    Ok(())
}

async fn handle_client(mut stream: TcpStream) -> Result<SocketAddr> {
    println!("↓[CONNECTED] device[{}]", stream.peer_addr()?);
    let mut buffer = [0; BUFFER_CAPACITY];
    let mut container_buffer: Vec<u8> = Vec::new();
    let mut bytes_read: usize;
    let mut current_buffer_size = 0_usize;
    
    loop {
        //check the buffer size is not more that 16MB in size to avoid DoS attack by using huge memory
        if container_buffer.len() > BUFFER_DATA_CAPACITY {
            return Err(TuringFeedsError::BufferDataCapacityFull)
        }

        bytes_read = stream.read(&mut buffer).await?;

        current_buffer_size = buffer[..bytes_read].len();

        if bytes_read == 0 {
            return Ok(stream.peer_addr()?);
        }

        match bincode::deserialize::<TuringTerminator>(&buffer[..bytes_read]) {
            Ok(TuringTerminator) => {
                // `0..bytes_read` Get the amount of bytes sent whether the buffer is full or not
                let to_stream = bincode::deserialize::<RepoCommands>(&container_buffer)?;
                
                //solve the problem and write to the stream
                //--let data_out = bincode::serialize::<OpsOutcome>(&repo_commands(to_stream).await)?;
                //--stream.write(&data_out).await?;
                //--stream.flush().await?;
                dbg!("DONE");
            },
            Err(_) => continue, // If an error occurs while deserializing, that means data is still being transmitted
        }

        // Append data to buffer after deserializing and determining that the current buffer data is not a `TuringTerminator`
        container_buffer.append(&mut buffer[..bytes_read].to_owned());
    }
}
 /*
async fn repo_commands(command: RepoCommands) -> OpsOutcome {
    // Return a DbOps::EncounteredErrors
    match command {
        RepoCommands::SuperUser(target) => {
            match superuser_commands(target).await {
                Ok(DbOps::Created) => OpsOutcome::Success(None),
                Ok(DbOps::RepoDeleted) => OpsOutcome::Success(None),
                Ok(DbOps::DbIntegrityConsistent) => OpsOutcome::Success(None),
                Ok(DbOps::DbIntegrityCorrupted) => 
                OpsOutcome::Failure(OperationErrors::Integrity(IntegrityErrors::IntegrityCorrupted)),
                Ok(DbOps::AlreadyExists) => OpsOutcome::Failure(OperationErrors::DbOps(DbOps::AlreadyExists)),
                Ok(DbOps::DbCreated) => OpsOutcome::Failure(OperationErrors::DbOps(DbOps::DbCreated)),
                Ok(DbOps::)
            };
            OpsOutcome::Failure(OperationErrors::Unspecified)
            //error_to_client(error).await;
        },
        // TODO Make these two enum variants work using privileges
        RepoCommands::Privileged(_) => OpsOutcome::Failure(OperationErrors::Unspecified),
        RepoCommands::UnPrivileged(_) => OpsOutcome::Failure(OperationErrors::Unspecified),
    }
}

*/

async fn superuser_commands(command: SuperUserTuringCommands) -> Result<(DbOps, Option<Vec<u8>>)> {
    match command {
        SuperUserTuringCommands::InitRepo => {
            match REPO.create().await {
                Ok(_) => Ok((DbOps::RepoCreated, None)),
                Err(error) => Err(error),
            }
        },
        SuperUserTuringCommands::DropRepo => {
            match REPO.drop_repo().await {
                Ok(FileOps::DeleteTrue) => Ok(DbOps::RepoDeleted),
                Err(error) => Err(error),
            }
        },
        SuperUserTuringCommands::ChecksumDatabase(target) => Ok(DbOps::Unspecified),
        SuperUserTuringCommands::ChecksumTable(target) => Ok(DbOps::Unspecified),
        SuperUserTuringCommands::CreateDatabase(target) => {
            let new_db = TuringFeedsDB::new().await
                .identifier(&target).await;
            // Insert DB to In-Memory 
            if let Some(ops) = REPO.get_mut() {
                match ops.memdb_add(new_db).await {
                    DbOps::AlreadyExists  => Ok(DbOps::AlreadyExists),
                    DbOps::DbCreated => {
                        // Commit to the logs
                        match REPO.commit().await {
                            Ok(_) => Ok(DbOps::DbCreated),
                            Err(error) => Err(error),
                        }
                    }
                    _ => Ok(DbOps::Unspecified),
                }
            }else {
                Ok(DbOps::EncounteredErrors("ONCE_CELL_DB_GLOBAL_STRUCTURE_UNINITIALIZED"))
            }
                
        },
        SuperUserTuringCommands::FetchDatabases(target) => {
            let data = REPO.list_memdbs().await;

            if data.is_empty() {
                Ok((DbOps::RepoEmpty, None))
            }else {
                Ok((DbOps::DbList, Some(data)))
            }
        },
        SuperUserTuringCommands::DropDatabase(target) => Ok(DbOps::DbDropped),
        SuperUserTuringCommands::CreateDocument(target_methods) => Ok(DbOps::DocumentCreated),
        SuperUserTuringCommands::FetchDocument(target_methods) => Ok(DbOps::DocumentFound),
        SuperUserTuringCommands::ModifyDocument(target_methods) => Ok(DbOps::DocumentModified),
        SuperUserTuringCommands::DeleteDocument(target_methods) => Ok(DbOps::DocumentDropped),
        SuperUserTuringCommands::Unspecified => Ok(DbOps::NotExecuted),
    }
}

async fn error_to_client(error: TuringFeedsError) -> DbOps {
    //make custom errorkinds of all possible errors in one enum instead of comparing then to strings
    match error {
        TuringFeedsError::IoError(inner) => DbOps::EncounteredErrors(inner.to_string()),
        TuringFeedsError::BincodeError(inner) => DbOps::EncounteredErrors(inner.to_string()),
        TuringFeedsError::RonDeError(inner) => DbOps::EncounteredErrors(inner.to_string()),
        TuringFeedsError::RonSerError(inner) => DbOps::EncounteredErrors(inner.to_string()),
        TuringFeedsError::Unspecified => DbOps::Unspecified,
    }
}
/* Future features for account rights
async fn priviliged_commands(command: PrivilegedTuringCommands) -> DbOps {
    match command {
        PrivilegedTuringCommands::CreateDatabase(target) => DbOps::DbCreated,
        PrivilegedTuringCommands::FetchDatabase(target) => DbOps::DbFound,
        PrivilegedTuringCommands::ModifyDatabase(target) => DbOps::DbModified,
        PrivilegedTuringCommands::DropDatabase(target) => DbOps::DbDropped,
        PrivilegedTuringCommands::Unspecified => DbOps::NotExecuted,
        ...
    }
}

async fn unprivileged_commands(command: UnprivilegedTuringCommands) -> DbOps {
    match command {
        UnprivilegedTuringCommands::CreateDocument(target_methods) => DbOps::DocumentCreated,
        UnprivilegedTuringCommands::FetchDocument(target_methods) => DbOps::DocumentFound,
        UnprivilegedTuringCommands::ModifyDocument(target_methods) => DbOps::DocumentModified,
        UnprivilegedTuringCommands::DeleteDocument(target_methods) => DbOps::DocumentDropped,
        UnprivilegedTuringCommands::Unspecified => DbOps::NotExecuted,
    }
}
*/

async fn errors_printable(error: TuringFeedsError) -> String {
    match error {
        TuringFeedsError::IoError(error) => format!("[STREAM ERROR]: {:?}", error.kind()),
        TuringFeedsError::RonDeError(error) => format!("[STREAM ERROR]: {}", error),
        TuringFeedsError::RonSerError(error) => format!("[STREAM ERROR]: {}", error),
        TuringFeedsError::BincodeError(error) => format!("[STREAM ERROR]: {}", error),
        TuringFeedsError::Unspecified => format!("[STREAM ERROR]: {}", "UNSPECIFIED"),
    }
}
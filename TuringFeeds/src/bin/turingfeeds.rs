#![forbid(unsafe_code)]

use async_std::{
    net::{TcpListener, TcpStream, SocketAddr},
    task,
    prelude::*,
    io::ErrorKind,
    stream::StreamExt
};
use tai64::TAI64N;
use std::net::Shutdown;
use custom_codes::DbOps;
use anyhow::Result;
use sled::Error as SledError;

use turingfeeds::{REPO, ErrorLogger};
use turingfeeds_helpers::{TuringCommands, OpsErrors};

const ADDRESS: &str = "127.0.0.1:43434";
const BUFFER_CAPACITY: usize = 64 * 1024; //16Kb
const BUFFER_DATA_CAPACITY: usize = 1024 * 1024 * 16; // Db cannot hold data more than 16MB in size



#[async_std::main]
async fn main() -> anyhow::Result<()> {

    match TcpListener::bind(ADDRESS).await {
        Ok(listener) => {
            REPO.repo_init().await;

            println!("Listening on Address: {}", listener.local_addr()?);
            while let Some(stream) = listener.incoming().next().await {
                let stream = stream?;
                task::spawn(async {
                    match handle_client(stream).await {
                        Ok(addr) => {
                            println!("x[TERMINATED] device[{}:{}]", addr.ip(), addr.port())
                        },
                        Err(error) => {
                            task::spawn(async move {
                                ErrorLogger::new(error).await;
                            });
                        },
                    }
                });
            }
        },
        Err(error) => {
            eprintln!("[TAI64N::<{:?}>] - [Tdb::<ERROR CONNECTING TO URI `tcp://localhost:43434` >] - [ErrorKind - {:?}]", TAI64N::now(), error.kind());
            std::process::exit(1)
        }
    }


    Ok(())
}

async fn handle_client(mut stream: TcpStream) -> Result<SocketAddr> {
    println!("↓[CONNECTED] device[{}]", stream.peer_addr()?);
    let mut header: [u8; 8] = [0; 8];

    let mut buffer = [0; BUFFER_CAPACITY];
    let mut container_buffer: Vec<u8> = Vec::new();
    let mut bytes_read: usize;

    stream.read(&mut header).await?;
    //Get the length of the data first
    let stream_byte_size = usize::from_le_bytes(header);
    let mut current_buffer_size = 0_usize;
    
    loop {
        //check the buffer size is not more that 16MB in size to avoid DoS attack by using huge memory
        if container_buffer.len() > BUFFER_DATA_CAPACITY {
            return Err(anyhow::Error::new(OpsErrors::BufferCapacityExceeded16Mb))
        }

        bytes_read = stream.read(&mut buffer).await?;

        if bytes_read == 0 {
            let peer = stream.peer_addr()?;
            //Shutdown the TCP address
            stream.shutdown(Shutdown::Both)?;
            // Terminate the stream if the client terminates the connection by sending 0 bytes
            return Ok(peer);
        }

        // Add the new buffer length to the current buffer size
        current_buffer_size += buffer[..bytes_read].len();

        if current_buffer_size == stream_byte_size {
            // Ensure that the data is appended before being deserialized by bincode
            container_buffer.append(&mut buffer[..bytes_read].to_owned());
            let data = bincode::deserialize::<TuringCommands>(&container_buffer).unwrap();

            let ops = process_command(data).await;
            handle_response(&mut stream, ops).await?;
            
        }
        // Append data to buffer
        container_buffer.append(&mut buffer[..bytes_read].to_owned());
    }
}

async fn handle_response(stream: &mut TcpStream, ops: DbOps) -> Result<()> {
    let ops_to_bytes = bincode::serialize::<DbOps>(&ops)?;
    stream.write(&ops_to_bytes.len().to_le_bytes()).await?;    
    stream.write(&ops_to_bytes).await?;
    stream.flush().await?;
    
    Ok(())
}

#[derive(Debug)]
enum EitherOps {
    OpsKind(DbOps),
    IoKind(ErrorKind),
}

/// Handle both std::io::Error and sled::Error::Io(error) with `EitherOps::IoKind`
/// In order to choose the `DbOps` response from I/O errors and other enum variant 
/// should handle other errors into `DbOps::EncounteredErrors(String)`
async fn get_errors(error: anyhow::Error) -> EitherOps {
    if let Some(ioerror) = error.root_cause().downcast_ref::<std::io::Error>() {
        EitherOps::IoKind(ioerror.kind())
    }else if let Some(sled_error) = error.root_cause().downcast_ref::<sled::Error>() {
        match sled_error {
            SledError::CollectionNotFound(_) => EitherOps::OpsKind(DbOps::EncounteredErrors("SLED_ERROR_COLLECTION_NOT_FOUND".into())),
            SledError::Unsupported(inner) => EitherOps::OpsKind(DbOps::EncounteredErrors(format!("SLED_ERROR_UNSURPOTED_[{:?}]", inner))),
            SledError::ReportableBug(inner) => EitherOps::OpsKind(DbOps::EncounteredErrors(format!("SLED_ERROR_UNSURPOTED_[{:?}]", inner))),
            SledError::Io(inner) => EitherOps::IoKind(inner.kind()),
            SledError::Corruption {at} => EitherOps::OpsKind(DbOps::EncounteredErrors(format!("SLED_ERROR_UNSURPOTED_[{:?}]", at))),
        }
    }else {
        EitherOps::OpsKind(DbOps::EncounteredErrors(error.to_string()))
    }
}

async fn repo_reponse_transform(values: Result<DbOps>) -> DbOps {
    match values {
        Ok(ops) => ops,
        Err(error) => {
            match get_errors(error).await {
                EitherOps::IoKind(inner) => {
                    if inner == ErrorKind::NotFound { DbOps::RepoNotFound }
                    else if inner == ErrorKind::PermissionDenied { DbOps::PermissionDenied }
                    else if inner == ErrorKind::AlreadyExists { DbOps::RepoAlreadyExists }
                    else { DbOps::EncounteredErrors(format!("REPO_I/O_ERROR_{:?}", inner)) }                         
                },
                EitherOps::OpsKind(inner) => DbOps::EncounteredErrors(format!("REPO_I/O_ERROR_{:?}", inner)),
            }
        }
    }
}

async fn db_reponse_transform(values: Result<DbOps>) -> DbOps {
    match values {
        Ok(ops) => ops,
        Err(error) => {
            match get_errors(error).await {
                EitherOps::IoKind(inner) => {
                    if inner == ErrorKind::NotFound { DbOps::DbNotFound }
                    else if inner == ErrorKind::PermissionDenied { DbOps::PermissionDenied }
                    else if inner == ErrorKind::AlreadyExists { DbOps::DbAlreadyExists }
                    else { DbOps::EncounteredErrors(format!("DB_I/O_ERROR_{:?}", inner)) }                         
                },
                EitherOps::OpsKind(inner) => DbOps::EncounteredErrors(format!("DB_I/O_ERROR_{:?}", inner)),
            }
        }
    }
}

async fn document_reponse_transform(values: Result<DbOps>) -> DbOps {
    match values {
        Ok(ops) => ops,
        Err(error) => {
            match get_errors(error).await {
                EitherOps::IoKind(inner) => {
                    if inner == ErrorKind::NotFound { DbOps::DocumentNotFound }
                    else if inner == ErrorKind::PermissionDenied { DbOps::PermissionDenied }
                    else if inner == ErrorKind::AlreadyExists { DbOps::DocumentAlreadyExists }
                    else { DbOps::EncounteredErrors(format!("DOCUMENT_I/O_ERROR_{:?}", inner)) }                         
                },
                EitherOps::OpsKind(inner) => DbOps::EncounteredErrors(format!("DOCUMENT_I/O_ERROR_{:?}", inner)),
            }
        }
    }
}

// TODO Detect whether I/O errors occur at the Repo, Db or Document level or their commit files

async fn field_reponse_transform(values: Result<DbOps>) -> DbOps {
    match values {
        Ok(ops) => ops,
        Err(error) => {
            match get_errors(error).await {
                EitherOps::IoKind(inner) => {
                    if inner == ErrorKind::NotFound { DbOps::FieldNotFound }
                    else if inner == ErrorKind::PermissionDenied { DbOps::PermissionDenied }
                    else if inner == ErrorKind::AlreadyExists { DbOps::FieldAlreadyExists }
                    else { DbOps::EncounteredErrors(format!("FIELD_I/O_ERROR_{:?}", inner)) }                         
                },
                EitherOps::OpsKind(inner) => DbOps::EncounteredErrors(format!("FIELD_I/O_ERROR_{:?}", inner)),
            }
        }
    }
}

async fn process_command(data: TuringCommands) -> DbOps { 
    match data {
        TuringCommands::RepoCreate => {
            repo_reponse_transform(REPO.repo_create().await).await
        },
        TuringCommands::RepoDrop => {
            repo_reponse_transform(REPO.repo_drop().await).await
        },
        TuringCommands::DbCreate(db_options) => {
            db_reponse_transform(REPO.db_create(&db_options).await).await
        },
        TuringCommands::DbRead(db_options) => {
            REPO.db_read(&db_options).await
        },
        TuringCommands::DbList => {
            REPO.db_list().await
        },
        TuringCommands::DbDrop(db_options) => {
            db_reponse_transform(REPO.db_drop(&db_options).await).await
        },
        TuringCommands::DocumentCreate(document_options) => {
            document_reponse_transform(REPO.document_create(document_options).await).await
        },
        TuringCommands::DocumentRead(document_options) => {
            REPO.document_read(document_options).await
        },
        TuringCommands::DocumentDrop(document_options) => {
            document_reponse_transform(REPO.document_drop(document_options).await).await
        },
        TuringCommands::FieldInsert(field_options) => {
            field_reponse_transform(REPO.field_insert(field_options).await).await
        },
        TuringCommands::FieldRead(field_options) => {
            field_reponse_transform(REPO.field_get(field_options).await).await
        },
        TuringCommands::FieldModify(field_options) => {
            field_reponse_transform(REPO.field_update(field_options).await).await
        },
        TuringCommands::FieldRemove(field_options) => {
            field_reponse_transform(REPO.field_drop(field_options).await).await
        },
    }
}
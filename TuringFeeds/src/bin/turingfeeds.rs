#![forbid(unsafe_code)]

use async_std::{
    net::{TcpListener, TcpStream, SocketAddr},
    task,
    prelude::*,
    io::ErrorKind,
    sync::{Arc, Mutex},
};
use custom_codes::{DbOps, FileOps};

use turingfeeds::{Result, FieldMetadata, Fields, Tdb, TuringFeeds, Documents};
use turingfeeds_helpers::{OpsOutcome, TuringFeedsError, DocumentMethods, RepoCommands, PrivilegedTuringCommands, UnprivilegedTuringCommands, SuperUserTuringCommands, OperationErrors, IntegrityErrors, TuringTerminator};

const ADDRESS: &str = "127.0.0.1:43434";
const BUFFER_CAPACITY: usize = 64 * 1024; //16Kb
const BUFFER_DATA_CAPACITY: usize = 1024 * 1024 * 16; // Db cannot hold data more than 16MB in size


// Just create normal hashmaps and then take 
// (Arc<Mutex<ReadHandle<&'static str, Box<TuringFeeds>>>>, Arc<Mutex<WriteHandle<&'static str, Box<TuringFeeds>>>>) = evmap::new()


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
    /*match REPO.set(TuringFeeds::new().await.init().await?) {
        Ok(_) => (),
        Err(error) => { eprintln!("{:?}", error); panic!(); }
    }  

    let doc = TFDocument::new().await
        .data(vec![9]).await;
    
    repo_inner_value().await.memdb_doc_create("Data1", "doc1", doc.clone()).await;
    repo_inner_value().await.memdb_doc_create("Data1", "doc1", doc.clone()).await;
    repo_inner_value().await.memdb_doc_create("Data1", "doc3", doc.clone()).await;
    repo_inner_value().await.memdb_doc_create("tttt", "doc3", doc.clone()).await;
    */

    //dbg!(&REPO);
    let mut fields1 = FieldMetadata::new().await;

    fields1.update_modified_time().await;

    let fields2 = FieldMetadata::new().await;
    let mut foo2 = Fields::new().await;
    foo2.insert("field_swap", fields2).await;

    let mut foo = Fields::new().await;
    
    let mut tf = TuringFeeds::new().await;
    //tf.create().await?;
    let mut documents = Documents::new().await;
    documents.insert("doc3", foo).await;
    let mut db = Tdb::memdb_new().await;
    db.db_create("db3").await?;
    db.memdb_insert("db3", documents).await;
    db.db_commit("db3").await?;
    tf.init().await;

    dbg!(tf);



    //foo.insert_field("foo_field1", fields1).await;
    //foo.insert_field("foo_field2", fields2).await;


    //dbg!(&foo.update_field("foo_field1").await);

    /*match TcpListener::bind(ADDRESS).await {
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
    }*/

    

    Ok(())
}

/*
#[async_std::main]
async fn main() -> Result<()> {
    // Check if database repository exists, if not exit with an error
    match REPO.set(TuringFeeds::new().await.init().await?) {
        Ok(_) => (),
        Err(error) => { eprintln!("{:?}", error); panic!(); }
    }  
    
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
}*/

async fn handle_client(mut stream: TcpStream) -> Result<SocketAddr> {
    println!("↓[CONNECTED] device[{}]", stream.peer_addr()?);
    let mut buffer = [0; BUFFER_CAPACITY];
    let mut container_buffer: Vec<u8> = Vec::new();
    let mut bytes_read: usize;
    
    loop {
        //check the buffer size is not more that 16MB in size to avoid DoS attack by using huge memory
        if container_buffer.len() > BUFFER_DATA_CAPACITY {
            return Err(TuringFeedsError::BufferDataCapacityFull)
        }

        bytes_read = stream.read(&mut buffer).await?;

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
                // Send the terminator header to the client
                //--bincode::serialize::<OpsOutcome>(&TuringTerminator).await?;
                //--stream.flush().await?;
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
/*
async fn superuser_commands(command: SuperUserTuringCommands) -> Result<(DbOps, Option<Vec<u8>>)> {
    match command {
        SuperUserTuringCommands::InitRepo => {
            match repo_inner_value().await.create().await {
                Ok(_) => Ok((DbOps::RepoCreated, None)),
                Err(error) => Err(error),
            }
        },
        SuperUserTuringCommands::DropRepo => {
            match repo_inner_value().await.drop_repo().await {
                Ok(FileOps::DeleteTrue) => Ok((DbOps::RepoDeleted, None)),
                Err(error) => Err(error),
            }
        },
        SuperUserTuringCommands::ChecksumDatabase(target) => Ok((DbOps::Unspecified, None)),
        SuperUserTuringCommands::ChecksumTable(target) => Ok((DbOps::Unspecified, None)),
        SuperUserTuringCommands::CreateDatabase(target) => {
            let new_db = TuringFeedsDB::new().await
                .identifier(&target).await;
            // Insert DB to In-Memory 
            match repo_inner_value().await.memdb_add(new_db).await {
                Ok((DbOps::AlreadyExists, _))  => Ok((DbOps::AlreadyExists, None)),
                Ok((DbOps::DbCreated, _)) => {
                    // Commit to the logs
                    match repo_inner_value().await.commit().await {
                        Ok(_) => Ok((DbOps::DbCreated, None)),
                        Err(error) => Err(error),
                    }
                }
                _ => Ok((DbOps::Unspecified, None)),
            }
                //Ok(DbOps::EncounteredErrors("ONCE_CELL_DB_GLOBAL_STRUCTURE_UNINITIALIZED"))                
        },
        SuperUserTuringCommands::FetchDatabases(target) => {
            let data = repo_inner_value().await.list_memdbs().await;

            if data.is_empty() {
                Ok((DbOps::RepoEmpty, None))
            }else {
                Ok((DbOps::DbList, Some(vec_string_to_u8(data).await)))
            }
        },
        SuperUserTuringCommands::DropDatabase(target) => Ok((repo_inner_value().await.memdb_rm(&target).await?, None)),
        SuperUserTuringCommands::CreateDocument(target_methods) => Ok(DbOps::DocumentCreated),
        SuperUserTuringCommands::InsertField(target_methods) => Ok(DbOps::DocumentInserted),
        SuperUserTuringCommands::FetchDocument(target_methods) => Ok(DbOps::DocumentFound),
        SuperUserTuringCommands::ModifyDocument(target_methods) => Ok(DbOps::DocumentModified),
        SuperUserTuringCommands::DeleteDocument(target_methods) => Ok(DbOps::DocumentDropped),
        SuperUserTuringCommands::Unspecified => Ok((DbOps::NotExecuted, None)),
    }
}
*/
async fn vec_string_to_u8(values: Vec<String>) -> Vec<u8> {
    let mut converted: Vec<u8> = Vec::new();

    for (index, value) in values.iter().enumerate() {
        converted.extend_from_slice(value.as_bytes());
        // Remove value to prevent too much memory usage from maintaining two vectors
        // values is now owned and should go out of scope
        //values.remove(index);
    }

    converted
}

async fn error_to_client(error: TuringFeedsError) -> DbOps {
    //make custom errorkinds of all possible errors in one enum instead of comparing then to strings
    match error {
        TuringFeedsError::IoError(inner) => DbOps::EncounteredErrors(inner.to_string()),
        TuringFeedsError::BincodeError(inner) => DbOps::EncounteredErrors(inner.to_string()),
        TuringFeedsError::RonDeError(inner) => DbOps::EncounteredErrors(inner.to_string()),
        TuringFeedsError::RonSerError(inner) => DbOps::EncounteredErrors(inner.to_string()),
        TuringFeedsError::Unspecified => DbOps::Unspecified,
        TuringFeedsError::BufferDataCapacityFull => DbOps::EncounteredErrors("BUFFER_CAPACITY_FULL".into()),
        TuringFeedsError::OsString(inner) => DbOps::EncounteredErrors(format!("{:?}", inner)),
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
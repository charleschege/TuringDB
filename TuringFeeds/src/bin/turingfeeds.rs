#![forbid(unsafe_code)]

use async_std::{
    net::{TcpListener, TcpStream, SocketAddr, ToSocketAddrs},
    task,
    prelude::*,
    io::{ErrorKind, BufReader},
    sync::{Arc, Mutex},
    stream::StreamExt
};
use custom_codes::{DbOps, FileOps};
use anyhow::Result;

use turingfeeds::REPO;
use turingfeeds_helpers::{TuringCommands, OpsErrors, TuringHeaders};

const ADDRESS: &str = "127.0.0.1:43434";
const BUFFER_CAPACITY: usize = 64 * 1024; //16Kb
const BUFFER_DATA_CAPACITY: usize = 1024 * 1024 * 16; // Db cannot hold data more than 16MB in size



#[async_std::main]
async fn main() -> anyhow::Result<()> {
    REPO.repo_init().await;

    match TcpListener::bind(ADDRESS).await {
        Ok(listener) => {
            println!("Listening on Address: {}", listener.local_addr()?);
            while let Some(stream) = listener.incoming().next().await {
                let stream = stream?;
                task::spawn(async {
                    match handle_client(stream).await {
                        Ok(addr) => {
                            println!("x[TERMINATED] device[{}:{}]", addr.ip(), addr.port())
                            // TODO Shutdown TCP connection
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
            // Terminate the stream if the client terminates the connection by sending 0 bytes
            return Ok(stream.peer_addr()?);
        }

        // Add the new buffer length to the current buffer size
        current_buffer_size += buffer[..bytes_read].len();

        if current_buffer_size == stream_byte_size {
            // Ensure that the data is appended before being deserialized by bincode
            container_buffer.append(&mut buffer[..bytes_read].to_owned());
            dbg!("ZERO BUFFER");
            dbg!(&container_buffer.len());
            let data = bincode::deserialize::<TuringCommands>(&container_buffer).unwrap();
            
            match data {
                TuringCommands::FieldInsert(inner) => { 
                    dbg!( std::str::from_utf8(&inner.data).unwrap());
                 },
                _ => { dbg!("ELSE COMMAND"); },
            }
        }
        // Append data to buffer
        container_buffer.append(&mut buffer[..bytes_read].to_owned());
    }
}

fn spawn_and_log_error<F>(fut: F) -> task::JoinHandle<()> 
    where F: Future<Output = Result<()>> + Send + 'static,
{
    task::spawn(async move {
        if let Err(e) = fut.await {
            eprintln!("{}", e)
        }
    })
}
/*

async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        println!("↓[CONNECTED] device[{}]", stream.peer_addr()?);
        let handle = task::spawn(connection_loop(stream));
        

    }

    Ok(())
}

async fn connection_loop(stream: TcpStream) -> Result<()> {
    let reader = BufReader::new(&stream);
    let mut lines = reader.lines();

    let name = match lines.next().await {
        None => "peer disconnected immediately".to_owned(),
        Some(line) => line?,
    };

    println!("name = {:?}", name);

    while let Some(line) = lines.next().await {
        let line = line?;
        let (dest, msg) = match line.find(':') {
            None => continue,
            Some(idx) => (&line[..idx], line[idx + 1 ..].trim()),
        };

        let dest: Vec<String> = dest.split(",").map(|name| name.trim().to_string()).collect();
        let msg: String = msg.to_string();
    }

    Ok(())
}

*/

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
}*/
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

/*


    let insert_data = DocumentMethods{
        db: "db0".into(),
        document: "doc0_db0".into(),
        field: "field_db0".into(),
        data: vec![16],
    };

    let insert_data2 = DocumentMethods{
        db: "db0".into(),
        document: "doc0_db0".into(),
        field: "field_db1".into(),
        data: vec![8],
    };

    let update_data2 = DocumentMethods{
        db: "db0".into(),
        document: "doc0_db0".into(),
        field: "field_db1".into(),
        data: vec![64],
    };

    let tf = TuringFeeds::new();
    tf.repo_init().await;
    //dbg!(&tf.repo_create().await);
    dbg!(&tf.db_create("db3").await);
    dbg!(&tf.db_list().await);
    dbg!(&tf.db_read("db0").await);
    dbg!(&tf.document_create("db0", "doc2_db0").await);
    dbg!(&tf.document_create("db0", "doc4_db0").await);
    dbg!(&tf.db_read("db0").await);
    dbg!(&tf.document_read("db0", "doc1_db0").await);
    dbg!(&tf.field_insert(insert_data2).await);
    dbg!(&tf.document_read("db0", "doc0_db0").await);

    let field = turingfeeds_helpers::FieldLite {
        db: "db0".into(),
        document: "doc0_db0".into(),
        field: "field_db1".into(),
    };

    dbg!(&tf.field_get(field).await);

    let field_to_drop = turingfeeds_helpers::FieldLite {
        db: "db0".into(),
        document: "doc0_db0".into(),
        field: "field_db1".into(),
    };

    dbg!(&tf.field_drop(field_to_drop.clone()).await);
    dbg!(&tf.field_get(field_to_drop).await);

    */
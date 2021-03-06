#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! TuringDB-Server is a key-value database written using async code and backed by `sled.rs`  embedded key/value store.
//! This is the server version, the library can be found by searching crates-io for `turingdb`
//! or checking under the Github repository https://github.com/charleschege/TuringDB/TuringDB/
//!
//!
//! This codebase uses `sled` as the underlying key/value store and builds upon that
//! to provide other functionality like
//!
//! 1. in-memory keys,
//! 2. async-locks for increased acid guarantees
//! 3. Insert operations will fail if a key already exists, use `modify()` method on a key to change its value
//! 4. in-memory locks to ensure that document locks are not dropped until the application is halted
//!
//! Some features that are under development include
//!
//! 1. Replication
//! 2. Multi-cluster queries
//! 3. Changefeeds without polling, inspired by RethinkDB
//! 4. Running the server as a daemon
//! 5. Loggin
//!
//! To install the server, run `cargo install turingdb-server`
//!
//! To run the server, run `turingdb-server` from a terminal

use anyhow::Result;
use async_dup::Arc;
use async_net::{TcpListener, TcpStream};
use custom_codes::DbOps;
use futures_lite::*;
use smol::Task;
use std::net::{Shutdown, SocketAddr};
use turingdb::TuringEngine;
use turingdb_helpers::{to_op, TuringOp};

mod repo_query;
use repo_query::*;

mod db_query;
use db_query::*;

mod document_query;
use document_query::*;

mod field_query;
use field_query::*;

mod errors;

const BUFFER_CAPACITY: usize = 64 * 1024; //16Kb
const BUFFER_DATA_CAPACITY: usize = 1024 * 1024 * 16; // Db cannot hold data more than 16MB in size

// FIXME Create a heartbeat of 100ms to check for when a repository is deliberately manipulated in the
// file system by the OS. Or acquire a lock to prevent modification by another process
//FIXME 1. ENABLE LISTENING FOR SIGNIT and other SIGNALS OVER CTRL-C AND NETWORKED SIGNALS OVER 4340
//FIXME 2. ENABLE RECORDING OF UNDERGOING OPERATIONS
//FIXME 5. LOGGING OF ERRORS
fn main() -> anyhow::Result<()> {
    // Initialize here to prevent issues with borrowing
    let storage = Arc::new(TuringEngine::new());

    smol::run(async {
        let storage = Arc::clone(&storage);
        match storage.repo_init().await {
            Ok(_) => (),
            Err(e) => {
                eprintln!("[TuringDB::<INIT>::(ERROR)-{:?}]", e); //FIXME log!()
                std::process::exit(1);
            }
        };

        let listener = TcpListener::bind("127.0.0.1:4343").await?;
        println!("Listening on {}", listener.local_addr()?);

        while let Some(stream) = listener.incoming().next().await {
            let stream = stream?;
            let storage = Arc::clone(&storage);

            Task::spawn(async move {
                match handle_client(stream, storage).await {
                    Ok(addr) => {
                        println!("x[TERMINATED] device[{}:{}]", addr.ip(), addr.port())
                        //FIXME log!()
                    }
                    Err(error) => {
                        eprintln!("{:?}", error); //FIXME log!()
                    }
                }
            })
            .await;
        }

        Ok(())
    })
}

async fn handle_client(mut stream: TcpStream, storage: Arc<TuringEngine>) -> Result<SocketAddr> {
    println!("↓[CONNECTED] device[{}]", stream.peer_addr()?);

    let mut buffer = [0; BUFFER_CAPACITY];
    let mut container_buffer: Vec<u8> = Vec::new();
    let mut bytes_read: usize;

    loop {
        //check the buffer size is not more that 16MB in size to avoid DoS attack by using huge memory
        if container_buffer.len() > BUFFER_DATA_CAPACITY {
            handle_response(
                &mut stream,
                DbOps::EncounteredErrors(
                    "[TuringDB::<GLOBAL>::(ERROR)-BUFFER_CAPACITY_EXCEEDED_16MB]".into(),
                ),
            )
            .await?;
        }

        bytes_read = stream.read(&mut buffer).await?;

        if bytes_read == 0 {
            let peer = stream.peer_addr()?;
            //Shutdown the TCP address
            stream.shutdown(Shutdown::Both)?;
            // Terminate the stream if the client terminates the connection by sending 0 bytes
            return Ok(peer);
        }

        // Check if the current stream is less than the buffer capacity, if so all data has been received
        if buffer[..bytes_read].len() < BUFFER_CAPACITY {
            // Ensure that the data is appended before being deserialized by bincode
            container_buffer.append(&mut buffer[..bytes_read].to_owned());
            let op = to_op(&[container_buffer[0]]);
            let op_result = process_op(&op, storage.clone(), &container_buffer[1..]).await;
            handle_response(&mut stream, op_result).await?;
        }
        // Append data to buffer
        container_buffer.append(&mut buffer[..bytes_read].to_owned());
    }
}

async fn process_op(op: &TuringOp, storage: Arc<TuringEngine>, value: &[u8]) -> DbOps {
    match op {
        &TuringOp::RepoCreate => RepoQuery::create(storage).await,
        &TuringOp::RepoDrop => RepoQuery::drop(storage).await,
        &TuringOp::DbCreate => DbQuery::create(storage, value).await,
        &TuringOp::DbList => DbQuery::list(storage).await,
        &TuringOp::DbDrop => DbQuery::drop(storage, value).await,
        &TuringOp::DocumentCreate => DocumentQuery::create(storage, value).await,
        &TuringOp::DocumentList => DocumentQuery::list(storage, value).await,
        &TuringOp::DocumentDrop => DocumentQuery::drop(storage, value).await,
        &TuringOp::FieldInsert => FieldQuery::insert(storage, value).await,
        &TuringOp::FieldGet => FieldQuery::get(storage, value).await,
        &TuringOp::FieldRemove => FieldQuery::remove(storage, value).await,
        &TuringOp::FieldModify => FieldQuery::modify(storage, value).await,
        &TuringOp::FieldList => FieldQuery::list(storage, value).await,
        &TuringOp::NotSupported => DbOps::NotExecuted,
    }
}

async fn handle_response(stream: &mut TcpStream, ops: DbOps) -> Result<()> {
    let ops_to_bytes = bincode::serialize::<DbOps>(&ops)?;
    stream.write(&ops_to_bytes).await?;
    stream.flush().await?;

    Ok(())
}

/*let (signal_sender, signal_receiver) = signal_msg::new();
signal_sender.prepare_signals();

#[derive(Debug, PartialEq, Eq)]
enum State {
    Running,
    Terminate,
}
 let mut state = State::Running;

    match signal_receiver.listen() {
        Ok(signal) => {
            println!("Received {:?} Terminating....", signal);
            state = State::Terminate;
        },
        Err(e) => eprintln!("[SIGNAL_MSG_ERROR] - {:?}", e), //FIXME
    }*/

/*
/// Create a new repository/directory that contains the databases
async fn create_ops_log_file(&self) -> Result<()> {
    unblock!(OpenOptions::new()
        .create_new(true)
        .write(true)
        .open("TuringDB_Repo/ops.log"))?;
    Ok(())
}
/// Create a new repository/directory that contains the databases
async fn create_errors_log_file(&self) -> Result<()> {
    unblock!(OpenOptions::new()
        .create_new(true)
        .write(true)
        .open("TuringDB_Repo/errors.log"))?;

    Ok(())
}
*/

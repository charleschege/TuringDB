#![forbid(unsafe_code)]

use async_std::{
    task,
    fs::{File, OpenOptions},
    net::{TcpListener, TcpStream},
    io::{prelude::*},
};


/// When using a queue to log data, ensure that the queue work completes before the db shuts down
/// Otherwise, if a user forcefully shuts down the db, abort queue work
/// Use difference to update logs
/// Add access rights and prevent adding rights without a superuser and access key
/// Store access keys and blake2b hashes for databases in zbox
/// Encrypt the database

fn main() {
    
}
#![forbid(unsafe_code)]

use async_std::{
    fs::{File, OpenOptions},
    net::{TcpListener, TcpStream, SocketAddr},
    task,
    prelude::*,
    io::{prelude::*, Read, BufReader},
};

use turingfeeds::{Result, TFDocument, TuringFeeds, TuringFeedsDB, TuringFeedsError};
use turingfeeds_helpers::{DocumentMethods, TuringCommand};

const ADDRESS: &str = "127.0.0.1:43434";

#[async_std::main]
async fn main() -> Result<()> {
    // Check if database repository exists, if not exit with an error
    let mut db = TuringFeeds::new().await;
    db.init().await?;

    /*let data = TuringFeedsDB::new().await.identifier("Data1").await;
    let data2 = TuringFeedsDB::new().await.identifier("Data2").await;
    let data3 = TuringFeedsDB::new().await.identifier("Data3").await;

    let data4 = TuringFeedsDB::new().await.identifier("Data3").await;

    db.memdb_add(data).await;
    db.memdb_add(data2).await;
    db.memdb_add(data3).await;
    dbg!(db.memdb_add(data4).await);
    dbg!(db);
    db.commit().await?;*/

    match TcpListener::bind(ADDRESS).await {
        Ok(listener) => {
            println!("Listening on Address: {}", listener.local_addr()?);
            while let Some(stream) = listener.incoming().next().await {
                let stream = stream?;
                task::spawn(async {
                    match handle_client(stream).await {
                        Ok(addr) => println!("[TERMINATED] ip({}) port({})", addr.ip(), addr.port()),
                        Err(error) => eprintln!("[STREAM ERROR]: {}", error),
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
    println!("Incoming stream from to {}", stream.peer_addr()?);
    let mut buffer = [0; 1024];
    let data_header = b"+----- ECHOOOOO -----+ \n";
    let data_footer = b"+--------------------+ \n\r";

    loop {
        let bytes_read = stream.read(&mut buffer).await?; // Get the amount of bytes sent whether the buffer is full or not
        if bytes_read == 0 {
            return Ok(stream.peer_addr()?);
        }
        
        let to_stream = bincode::deserialize::<TuringCommand>(&buffer[0..bytes_read])?;
        stream.write(data_header).await?;
        dbg!(to_stream);
        //stream.write(&to_stream.into_bytes()).await?;
        //stream.write(&buffer[..foo().await]).await?;
        //stream.write(&buffer[..bytes_read]).await?;
        stream.write(data_footer).await?;
    }
}


/*
async fn handle_client(mut stream: TcpStream) -> Result<SocketAddr> {
    let mut buffer = [0; 1024];
    let data_header = b"+----- ECHOOOOO -----+ \n";
    let data_footer = b"+--------------------+ \n\r";

    loop {
        let bytes_read = stream.read(&mut buffer).await?; // Get the amount of bytes sent whether the buffer is full or not
        if bytes_read == 0 {
            return Ok(stream.peer_addr()?);
        }
        //stream.peek(&mut buf).await?;
        stream.write(data_header).await?;
        let to_stream = String::from_utf8(buffer[0..bytes_read].to_vec()).unwrap().trim().to_owned() + &foo().await + "\n";
        stream.write(&to_stream.into_bytes()).await?;
        //stream.write(&buffer[..foo().await]).await?;
        //stream.write(&buffer[..bytes_read]).await?;
        stream.write(data_footer).await?;
    }
}
*/
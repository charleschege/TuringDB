use async_std::{
    io::prelude::*,
    net::TcpStream,
};
use std::str;
use turingfeeds_helpers::{UnprivilegedTuringCommands, DocumentMethods};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
const ADDRESS: &str = "127.0.0.1:43434";
const BUFFER_CAPACITY: usize = 64 * 1024;


#[async_std::main]
async fn main() -> Result<()> {
    let mut stream = TcpStream::connect(ADDRESS).await?;
    let mut buffer = [0; BUFFER_CAPACITY];

    let data = DocumentMethods::new().await
        .add_db("Turing1").await
        .add_document("Doc1").await
        .add_data("data".as_bytes().to_vec()).await;
    
    stream.write(&bincode::serialize::<UnprivilegedTuringCommands>(&UnprivilegedTuringCommands::CreateDocument(data))?).await?;
    let bytes_read = stream.read(&mut buffer).await?;
    
    println!("{:?}", bincode::deserialize::<custom_codes::DbOps>(&buffer[..bytes_read])?);
        
    Ok(())
}
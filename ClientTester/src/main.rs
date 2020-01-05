use async_std::{
    io::prelude::*,
    net::TcpStream,
};
use std::str;
use turingfeeds_helpers::TuringCommand;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
const ADDRESS: &str = "127.0.0.1:43434";
const BUFFER_CAPACITY: usize = 64 * 1024;


#[async_std::main]
async fn main() -> Result<()> {
    let mut stream = TcpStream::connect(ADDRESS).await?;
    let mut buffer = [0; BUFFER_CAPACITY];
    stream.write(&bincode::serialize::<TuringCommand>(&TuringCommand::CreateDatabase("db1".to_owned()))?).await?;
    let bytes_read = stream.read(&mut buffer).await?;
    
    println!("{:?}", bincode::deserialize::<TuringCommand>(&buffer[..bytes_read]).unwrap());
        
    Ok(())
}
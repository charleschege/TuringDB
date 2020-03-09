use async_std::{
    io::prelude::*,
    net::TcpStream,
};
use std::str;
use anyhow::Result;
use turingfeeds_helpers::{TuringCommands, TuringHeaders, DocumentOnly, FieldNoData, FieldWithData, OpsErrors};
use custom_codes::DbOps;

const ADDRESS: &str = "127.0.0.1:43434";
const BUFFER_CAPACITY: usize = 64 * 1024; //16Kb
const BUFFER_DATA_CAPACITY: usize = 1024 * 1024 * 16; // Db cannot hold data more than 16MB in size

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Foo(String);



#[async_std::main]
async fn main() -> Result<()> {
    let lorem = "Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptate dignissimos optio, illo commodi inventore fuga consequuntur laboriosam ex fugit nisi! Sequi nostrum consectetur iste laboriosam ipsa, iusto recusandae cumque consequuntur!Assumenda repellat mollitia amet nostrum a reiciendis, odit voluptas harum! Vero quidem eligendi pariatur quo, unde molestiae delectus eaque totam ex. Id eius laborum amet voluptatem, maxime nostrum voluptas tenetur!
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptate dignissimos optio, illo commodi inventore fuga consequuntur laboriosam ex fugit nisi! Sequi nostrum consectetur iste laboriosam ipsa, iusto recusandae cumque consequuntur!Assumenda repellat mollitia amet nostrum a reiciendis, odit voluptas harum! Vero quidem eligendi pariatur quo, unde molestiae delectus eaque totam ex. Id eius laborum amet voluptatem, maxime nostrum voluptas tenetur!
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptate dignissimos optio, illo commodi inventore fuga consequuntur laboriosam ex fugit nisi! Sequi nostrum consectetur iste laboriosam ipsa, iusto recusandae cumque consequuntur!Assumenda repellat mollitia amet nostrum a reiciendis, odit voluptas harum! Vero quidem eligendi pariatur quo, unde molestiae delectus eaque totam ex. Id eius laborum amet voluptatem, maxime nostrum voluptas tenetur!
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptate dignissimos optio, illo commodi inventore fuga consequuntur laboriosam ex fugit nisi! Sequi nostrum consectetur iste laboriosam ipsa, iusto recusandae cumque consequuntur!Assumenda repellat mollitia amet nostrum a reiciendis, odit voluptas harum! Vero quidem eligendi pariatur quo, unde molestiae delectus eaque totam ex. Id eius laborum amet voluptatem, maxime nostrum voluptas tenetur!
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptate dignissimos optio, illo commodi inventore fuga consequuntur laboriosam ex fugit nisi! Sequi nostrum consectetur iste laboriosam ipsa, iusto recusandae cumque consequuntur!Assumenda repellat mollitia amet nostrum a reiciendis, odit voluptas harum! Vero quidem eligendi pariatur quo, unde molestiae delectus eaque totam ex. Id eius laborum amet voluptatem, maxime nostrum voluptas tenetur!
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptate dignissimos optio, illo commodi inventore fuga consequuntur laboriosam ex fugit nisi! Sequi nostrum consectetur iste laboriosam ipsa, iusto recusandae cumque consequuntur!Assumenda repellat mollitia amet nostrum a reiciendis, odit voluptas harum! Vero quidem eligendi pariatur quo, unde molestiae delectus eaque totam ex. Id eius laborum amet voluptatem, maxime nostrum voluptas tenetur!";

    let mut stream = TcpStream::connect(ADDRESS).await?;

    let data = TuringCommands::FieldInsert(FieldWithData {
        db: "db0".into(),
        document: "doc0_db0".into(),
        field: "field0".into(),
        data: lorem.as_bytes().to_vec(),
    });

    //let buffer = bincode::serialize::<TuringCommands>(&TuringCommands::RepoRead)?;
    let data = bincode::serialize::<TuringCommands>(&data)?;
    dbg!(&data.len());
    
    
    stream.write(&data.len().to_le_bytes()).await?;
    dbg!("SENT HEADER");
    //stream.write(&bincode::serialize::<TuringHeaders>(&TuringHeaders::Terminator)?).await?;
    stream.write(&data).await?;
    dbg!("SENT DATA");
    

    stream.flush().await?;

    let mut buffer = [0; BUFFER_CAPACITY];
    let mut container_buffer: Vec<u8> = Vec::new();
    let mut bytes_read: usize;

    bytes_read = stream.read(&mut buffer).await?;
    dbg!(bytes_read);
    Ok(())
}
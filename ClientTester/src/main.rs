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

    let lorem_modify = "modified lorem";

    let repo_create = bincode::serialize::<TuringCommands>(&TuringCommands::RepoCreate)?;
    let repo_drop = bincode::serialize::<TuringCommands>(&TuringCommands::RepoDrop)?;
    let db_create = bincode::serialize::<TuringCommands>(&TuringCommands::DbCreate("db0".into()))?;
    let db_read = bincode::serialize::<TuringCommands>(&TuringCommands::DbRead("db0".into()))?;
    let db_list = bincode::serialize::<TuringCommands>(&TuringCommands::DbList("db0".into()))?;
    let db_drop = bincode::serialize::<TuringCommands>(&TuringCommands::DbDrop("db0".into()))?;
    let document_create = bincode::serialize::<TuringCommands>(&TuringCommands::DocumentCreate({
        DocumentOnly {
            db: "db0".into(),
            document: "doc0_db0".into(),
        }
    }))?;
    let document_read = bincode::serialize::<TuringCommands>(&TuringCommands::DocumentRead({
        DocumentOnly {
            db: "db0".into(),
            document: "doc0_db0".into(),
        }
    }))?;
    let document_drop = bincode::serialize::<TuringCommands>(&TuringCommands::DocumentDrop({
        DocumentOnly {
            db: "db0".into(),
            document: "doc0_db0".into(),
        }
    }))?;
    let field_create = bincode::serialize::<TuringCommands>(&TuringCommands::FieldInsert({
        FieldWithData {
            db: "db0".into(),
            document: "doc0_db0".into(),
            field: "field0".into(),
            data: lorem.as_bytes().to_vec(),
        }
    }))?;
    let field_read = bincode::serialize::<TuringCommands>(&TuringCommands::FieldRead({
        FieldNoData {
            db: "db0".into(),
            document: "doc0_db0".into(),
            field: "field0".into(),
        }
    }))?;
    let field_modify = bincode::serialize::<TuringCommands>(&TuringCommands::FieldModify({
        FieldWithData {
            db: "db0".into(),
            document: "doc0_db0".into(),
            field: "field0".into(),
            data: "barrrrrrrrrrrr".as_bytes().to_vec(),
        }
    }))?;
    let field_drop = bincode::serialize::<TuringCommands>(&TuringCommands::FieldRemove({
        FieldNoData {
            db: "db0".into(),
            document: "doc0_db0".into(),
            field: "field0".into(),
        }
    }))?;

    let mut stream = TcpStream::connect(ADDRESS).await?;
    
    
    stream.write(&field_read.len().to_le_bytes()).await?;
    stream.write(&field_read).await?;
    stream.flush().await?;


    let mut header: [u8; 8] = [0; 8];

    let mut buffer = [0; BUFFER_CAPACITY];
    let mut container_buffer: Vec<u8> = Vec::new();
    let mut bytes_read: usize;

    stream.read(&mut header).await?;

    //Get the length of the data first
    let stream_byte_size = usize::from_le_bytes(header);
    let mut current_buffer_size = 0_usize;
    
    loop {
        bytes_read = stream.read(&mut buffer).await?;

        // Add the new buffer length to the current buffer size
        current_buffer_size += buffer[..bytes_read].len();

        if current_buffer_size == stream_byte_size {
            // Ensure that the data is appended before being deserialized by bincode
            container_buffer.append(&mut buffer[..bytes_read].to_owned());
            let data = bincode::deserialize::<DbOps>(&container_buffer).unwrap();
            dbg!(&data);
            
            match data {
                DbOps::FieldContents(inner) => { dbg!(str::from_utf8(&inner)?); },
                _ => { dbg!(data); }
            }

            break;
        }
        // Append data to buffer
        container_buffer.append(&mut buffer[..bytes_read].to_owned());
    }

    // TODO Ensure client does not immediately terminate

    Ok(())
}
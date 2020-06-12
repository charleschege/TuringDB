#![forbid(unsafe_code)]
use tai64::TAI64N;
use std::{
    net::Shutdown,
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
};
use custom_codes::DbOps;
use anyhow::Result;
use simple_signal::{self, Signal};
use smol::{Async, Task};

const ADDRESS: &str = "127.0.0.1:43434";
const BUFFER_CAPACITY: usize = 64 * 1024; //16Kb
const BUFFER_DATA_CAPACITY: usize = 1024 * 1024 * 16; // Db cannot hold data more than 16MB in size

// [TAI64N::<{:?}>] - [Tdb::<ERROR CONNECTING TO URI `tcp://localhost:43434` >] - [ErrorKind - {:?}]", TAI64N::now(), error.kind());

fn main() -> anyhow::Result<()> {
    /// 1. ENABLE LISTENING FOR SIGNIT and other SIGNALS OVER CTRL-C AND NETWORKED SIGNALS OVER 4340
    /// 2. ENABLE RECORDING OF UNDERGOING OPERATIONS
    /// 3. E BIND TO ADDRESS
    /// 4. ENABLE TERMINATING OF CONNECTIONS FROM CLIENTS
    /// 5. LOGGING OF ERRORS

    let sigterm = Arc::new(AtomicBool::new(true));
    let sigterm_listener = Arc::clone(&sigterm);
    
    simple_signal::set_handler(
        &[Signal::Int, Signal::Term, Signal::Kill, Signal::Quit],
        move |signals| {
            println!("{:?}", signals);
            sigterm_listener.store(false, Ordering::SeqCst);
    });

    while sigterm.load(Ordering::SeqCst) {
        println!("Waiting for Ctrl-C...");
    }
    println!("Got it! Exiting...");

    Ok(())
}
/*
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
*/
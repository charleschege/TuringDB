## Turing·DB

<img src="TuringDB/Mockups/Logo.svg" alt="Turing·DB Logo" style="zoom:25%;" />

TuringDB is a database, written in Rust, that aims to be distributed and scaled horizontally. It aims to be a replacement for where you don't need a relational database or a schema. 

The database is backed by Sled key-value store

### Motivation

The motive behind this database is to have a key/value database with ACID properties, speed, type-safety, changefeeds without polling, multi-cluster queries and replication. Rust is the most qualified for speed, type-safety and compile-time checks. Furthermore, `sled.rs` has been used as the embedded key/value store for this database as it is lock-free, has fully ATOMIC operations, zero-copy reads, SSD optimized log-storage and is written in Rust, hence inherits all the sweet properties of the language.

#### Features

The database aims to be:

1. Really simple to use document database
2. Have lookup and range capabilities
3. Be partition tolerant and consistent
4. Offer real-time push capabilities without polling, inspired by RethinkDB changefeeds
5. Offer simple joins
6. Offer optional distributed capabilities backed by Raft consensus algorithm
7. Offer optional multi-cluster queries
8. Be small enough to use as embedded database
9. Be small and fast enough to be used on embedded devices or large servers
10. Be really fun to use

#### Features under development include

1. Replication
2. Multi-cluster queries
3. Changefeeds without polling, inspired by RethinkDB
4. JSON support

#### Server Usage 

1. **Install from crates-io**

    ```sh
    $ cargo install turingdb-server
    ```

2. **Start the server**

    ```sh
    $ turingdb-server
    ```

3. **Create a new cargo repository**

   ```sh
   $ cargo new my-app
   ```

4. **Edit `Cargo.toml` file**

    ```toml
    #[dependencies]
    turingdb-helpers = #add the latest version here
    bincode = #add the latest version
    async-std = #add latest version here
    anyhow = # add latest version
    custom_codes = #add latest vesion
    ```
    
    Alternatively you could use `cargo-edit` if it is already installed, instead of adding dependencies manually
    
    ```sh
    $ cargo add turingdb-helpers bincode async-std anyhow custom_codes
    ```
    
    
    
5. **Open `src/main.rs` file in an editor**

    ```rust
    use async_std::net::TcpStream;
    use async_std::io::prelude::*;
    use serde::{Serialize, Deserialize};
    use custom_codes::DbOps;
    
    const BUFFER_CAPACITY: usize = 64 * 1024; //16Kb
    const BUFFER_DATA_CAPACITY: usize = 1024 * 1024 * 16; // Db cannot hold data more than 16MB in size
    
    #[derive(Debug, Serialize, Deserialize)]
    struct DocumentQuery {
        db: String,
        document: Option<String>,
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    pub (crate) struct FieldQuery {
        db: String,
        document: String,
        field: String,
        payload: Option<Vec<u8>>,
    }
    
    #[async_std::main]
    async fn main() -> anyhow::Result<()> {
    	let db_create = "db0".as_bytes();
    	let mut packet = vec![0x02];
    	packet.extend_from_slice(&db_create);
    
        let mut buffer = [0; BUFFER_CAPACITY];
        let mut container_buffer: Vec<u8> = Vec::new();
        let mut bytes_read: usize;
        let mut current_buffer_size = 0_usize;
        
        let mut stream = TcpStream::connect("127.0.0.1:4343").await?;
        stream.write(&packet).await?;
    
        loop {
            bytes_read = stream.read(&mut buffer).await?;
            
    
            // Add the new buffer length to the current buffer size
            current_buffer_size += buffer[..bytes_read].len();
    
            // Check if the current stream is less than the buffer capacity, if so all data has been received
            if  buffer[..bytes_read].len() < BUFFER_CAPACITY {
                // Ensure that the data is appended before being deserialized by bincode
                container_buffer.append(&mut buffer[..bytes_read].to_owned());
    			dbg!(&container_buffer);
    
    			dbg!(bincode::deserialize::<DbOps>(&container_buffer).unwrap());
    		
    			break;
                
            }
            // Append data to buffer
            container_buffer.append(&mut buffer[..bytes_read].to_owned());        
        }
    
        Ok(())
    }
    ```

#### **Current query methods supported by the database**

1. **Repository Queries**

   - **`turingdb_helpers::RepoQuery::create()`** creates a new repository in the current directory
   - **`turingdb_helpers::RepoQuery::drop()`** drops a repository in the current directory

2. **Database Queries**

   - **`DbQuery::create()`** creates a new database in the repository

     ```rust
     use turingdb_helpers::DatabaseQuery;
     
     let mut foo = DatabaseQuery::new().await;
     foo
       .db("db_name").await
       .create().await;
     ```

     

   - **`DbQuery::drop()`** drops a database in the repository

     ```rust
     use turingdb_helpers::DatabaseQuery;
     
     let mut foo = DatabaseQuery::new().await;
     foo
     	.db("db_name").await
     	.drop().await;
     ```

     

   - **`DbQuery::list()`** list all database in the repository

     ```rust
     use turingdb_helpers::DatabaseQuery;
     
     let mut foo = Database::new().await;
     foo.drop().await;
     ```

3. **Document Queries**

   - **`DocumentQuery::create()`** create a document in the database

     ```rust
     use turingdb_helpers::DocumentQuery;
     
     let mut foo = DocumentQuery::new().await;
     foo
     	.db("db_name").await
     	.document("document_name").await
     	.create().await;
     ```

     

   - **`DocumentQuery::drop()`** drops a document in the database

     ```rust
     use turingdb_helpers::DocumentQuery;
     
     let mut foo = DocumentQuery::new().await;
     foo
     	.db("db_name").await
     	.document("document_name").await
     	.drop().await;
     ```

     

   - **`DocumentQuery::list()`** lists all documents in the database

     ```rust
     use turingdb_helpers::DocumentQuery;
     
     let mut foo = DocumentQuery::new().await;
     foo
     	.db("db_name").await
     	.list().await;
     ```

4. **Field Queries**

   - **`Field::set()`** create a field in a  document based on a key

     ```rust
     use turingdb_helpers::FieldQuery;
     
     let mut foo = FieldQuery::new().await;
     let data = "my_data_converted_into_bytes".as_bytes();
     foo
       .db("db_name").await
       .document("document_name").await
       .field("field_name").await
       .payload(data).await
       .set().await
     ```

     

   - **`Field::get()`** gets a field in a  document based on a key

     ```rust
     use turingdb_helpers::FieldQuery;
     
     let mut foo = FieldQuery::new().await;
     foo
       .db("db_name").await
       .document("document_name").await
       .field("field_name").await
       .get().await
     ```

     

   - **`Field::modify()`** updates a field in a  document based on a key

     ```rust
     use turingdb_helpers::FieldQuery;
     
     let mut foo = FieldQuery::new().await;
     let data = "my_new_data_converted_into_bytes".as_bytes();
     foo
       .db("db_name").await
       .document("document_name").await
       .field("field_name").await
       .payload(data).await
       .modify().await
     ```

     

   - **`Field::remove()`** remove a field in a  document based on a key

     ```rust
     use turingdb_helpers::FieldQuery;
     
     let mut foo = FieldQuery::new().await;
     foo
       .db("db_name").await
       .document("document_name").await
       .field("field_name").await
       .remove().await
     ```

     

   - **`Field::list()`** get all keys of fields

     ```rust
     use turingdb_helpers::FieldQuery;
     
     let mut foo = FieldQuery::new().await;
     foo
       .db("db_name").await
       .document("document_name").await
       .list().await
     ```

     

### **`Warning`**  

 A document cannot hold more that `16MiB` of data and if this threshold is exceeded, an error from the `custom_codes` crate `DbOps::EncounteredErrors([TuringDB::<GLOBAL>::(ERROR)-BUFFER_CAPACITY_EXCEEDED_16MB])`

#### Contributing

We follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct) in making contributions

#### License

All code contributions to this project must be licensed under Apache license

#### Acknowledgement

All libraries used in this project are subject to their own licenses

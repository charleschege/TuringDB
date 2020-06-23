#### Version 2.0.0-beta.1

1. Change library from `async-std` to `smol`
2. Remove `evmap` and substitute it with `dashmap` to reduce memory size. This effectively reduces the RAM usage by about 90%
3. Change the structure of the database
4. Move from using global variables to local variables with use of `Arc` instead
5. Hold sled database (a TuringDB document) as a file descriptor inside a hashmap to prevent opening and closing the file all the time
6. Unblock all the blocking file operations
7. Move away from storage of database and repo structure in files to initializing the repository by recursively walking the directory in search of databases and holding their contents in memory
8. Move all error handling away from the library
9. Move the server code into its new repository
10. Change all the dependencies to ones that can co-exist without bloat
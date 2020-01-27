## TuringFeeds

TuringFeeds is a simple, pure Rust database that aims to be distributed and scaled horizontally. It aims to be a replacement for SQLite where you don't need a relational database or a schema. 

The database is backed by Sled key-value store.

#### **Warning** 

```
The database is currently under development and is not yet suitable for production.
```

#### Features

The database aims to be:

1. Really simple to use document database
2. Have lookup and range capabilities
3. Be partition tolerant and consistent
4. Offer real-time push capabilities without polling, inspired by RethinkDB changefeeds
5. Offer simple joins
6. Offer distributed capabilities backed by Raft consensus algorithm
7. Offer multi-cluster queries
8. Be small enough to use as embedded database
9. Be small and fast enough to be used on embedded devices or large servers
10. Be really fun to use

#### Contributing

We follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct) in making contributions

#### License

All code contributions to this project must be licensed under Apache license

#### Acknowledgement

All libraries used in this project are subject to their own licenses
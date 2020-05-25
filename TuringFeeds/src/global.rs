#[derive(Debug)]
pub enum AccessRights {
    Table(String),
    Db(String),
}

#[derive(Debug)]
pub enum Role {
    SuperUser,
    Admin,
    SubAdmin,
    User,
}

#[derive(Debug)]
pub enum DbType {
    KeyValueStore,
    RealTimeFeeds,
}

use serde::{Deserialize, Serialize};
use tai64::TAI64N;

/// Contains the structure of a value represented by a key
///
/// `Warning:` This is serialized using bincode so deserialization should be done using same version of bincode
/// ```
/// #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
/// pub struct FieldData {
///     data: Vec<u8>,
///     created: TAI64N,
///     modified: TAI64N,
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct FieldData {
    data: Vec<u8>,
    created: TAI64N,
    modified: TAI64N,
}
/*
impl FieldData {
    /// Initializes a new `FieldData` struct
    pub fn new(value: &[u8]) -> FieldData {
        let current_time = TAI64N::now();

        Self {
            data: value.into(),
            created: current_time,
            modified: current_time,
        }
    }
    /// Updates a `FieldData` by modifying its time with a new `TAI64N` timestamp
    pub fn update(&mut self, value: &[u8]) -> &FieldData {
        self.data = value.into();
        self.modified = TAI64N::now();

        self
    }
}

    /// List all fields in a document
    pub async fn field_list(&self, db_name: &Path, doc_name: &Path) -> DbOps {
        if self.dbs.is_empty() {
            return DbOps::RepoEmpty;
        }

        if let Some(mut database) = self.dbs.get_mut(&OsString::from(db_name)) {
            if let Some(document) = database.value_mut().list.get_mut(&OsString::from(doc_name)) {
                if document.keys.is_empty() {
                    DbOps::DocumentEmpty
                } else {
                    let data = document.keys.iter().map(|key| key.to_vec()).collect();

                    DbOps::FieldList(data)
                }
            } else {
                DbOps::DocumentNotFound
            }
        } else {
            DbOps::DbNotFound
        }
    }
    /// Create a field with data
    pub async fn field_insert(
        &self,
        db_name: &Path,
        doc_name: &Path,
        field_name: &[u8],
        data: &[u8],
    ) -> Result<DbOps> {
        if self.dbs.is_empty() {
            return Ok(DbOps::RepoEmpty);
        }

        if field_name.is_empty() {
            return Ok(DbOps::EncounteredErrors(
                "[TuringDB::<FieldList>::(ERROR)-FIELD_NAME_EMPTY]".to_owned(),
            ));
        }

        if data.is_empty() {
            return Ok(DbOps::EncounteredErrors(
                "[TuringDB::<FieldList>::(ERROR)-DATA_FIELD_EMPTY]".to_owned(),
            ));
        }

        if let Some(mut database) = self.dbs.get_mut(&OsString::from(db_name)) {
            if let Some(document) = database.value_mut().list.get_mut(&OsString::from(doc_name)) {
                if document.keys.binary_search(&field_name.to_vec()).is_ok() {
                    Ok(DbOps::FieldAlreadyExists)
                } else {
                    document.fd.lock().await.insert(field_name, data)?;

                    Ok(DbOps::FieldInserted)
                }
            } else {
                Ok(DbOps::DocumentNotFound)
            }
        } else {
            Ok(DbOps::DbNotFound)
        }
    }
    /// Get a field
    pub async fn field_get(
        &self,
        db_name: &Path,
        doc_name: &Path,
        field_name: &[u8],
    ) -> Result<DbOps> {
        if self.dbs.is_empty() {
            return Ok(DbOps::RepoEmpty);
        }

        if let Some(mut database) = self.dbs.get_mut(&OsString::from(db_name)) {
            if let Some(document) = database.value_mut().list.get_mut(&OsString::from(doc_name)) {
                if document.keys.binary_search(&field_name.to_vec()).is_ok() {
                    match document.fd.lock().await.get(field_name)? {
                        Some(data) => Ok(DbOps::FieldContents(data.to_vec())),
                        None => Ok(DbOps::FieldNotFound),
                    }
                } else {
                    Ok(DbOps::FieldNotFound)
                }
            } else {
                Ok(DbOps::DocumentNotFound)
            }
        } else {
            Ok(DbOps::DbNotFound)
        }
    }
    /// Drop a field
    pub async fn field_remove(
        &self,
        db_name: &Path,
        doc_name: &Path,
        field_name: &[u8],
    ) -> Result<DbOps> {
        if self.dbs.is_empty() {
            return Ok(DbOps::RepoEmpty);
        }

        if let Some(mut database) = self.dbs.get_mut(&OsString::from(db_name)) {
            if let Some(document) = database.value_mut().list.get_mut(&OsString::from(doc_name)) {
                if let Ok(field_index) = document.keys.binary_search(&field_name.to_vec()) {
                    let sled_op = document.fd.lock().await.remove(field_name)?;

                    match sled_op {
                        Some(_) => {
                            document.keys.remove(field_index);
                            Ok(DbOps::FieldDropped)
                        }
                        None => Ok(DbOps::FieldNotFound),
                    }
                } else {
                    Ok(DbOps::FieldNotFound)
                }
            } else {
                Ok(DbOps::DocumentNotFound)
            }
        } else {
            Ok(DbOps::DbNotFound)
        }
    }
    /// Update a field
    pub async fn field_modify(
        &self,
        db_name: &Path,
        doc_name: &Path,
        field_name: &[u8],
        field_value: &[u8],
    ) -> Result<DbOps> {
        if self.dbs.is_empty() {
            return Ok(DbOps::RepoEmpty);
        }

        if let Some(mut database) = self.dbs.get_mut(&OsString::from(db_name)) {
            if let Some(document) = database.value_mut().list.get_mut(&OsString::from(doc_name)) {
                if document.keys.binary_search(&field_name.to_vec()).is_ok() {
                    let field_key: Vec<u8> = field_name.to_owned();
                    let stored_data;

                    let key_exists = document.fd.lock().await.get(&field_key)?;

                    match key_exists {
                        Some(data) => {
                            stored_data = data.to_vec();
                            let mut current_field_data =
                                bincode::deserialize::<FieldData>(&stored_data)?;
                            current_field_data.update(field_value);
                            let modified_field_data = bincode::serialize(&current_field_data)?;
                            match document
                                .fd
                                .lock()
                                .await
                                .insert(field_key, modified_field_data)?
                            {
                                Some(_) => Ok(DbOps::FieldModified),
                                // FIXME Decide what to do in case the field didnt exist
                                // Maybe push these to the database logs and alert DB Admin
                                None => Ok(DbOps::FieldInserted),
                            }
                        }
                        None => Ok(DbOps::FieldNotFound),
                    }
                } else {
                    Ok(DbOps::FieldNotFound)
                }
            } else {
                Ok(DbOps::DocumentNotFound)
            }
        } else {
            Ok(DbOps::DbNotFound)
        }
    }*/

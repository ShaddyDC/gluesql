use {
    crate::storage_error::StorageError, gluesql_core::prelude::Key, gluesql_core::result::Result,
};

// Note that we rely on table names containing no '/'.
// Given that we should only be called from Glue,
// where those should not be valid table names anyway
// that shouldn't be a problem.

pub(crate) fn generate_key(table_name: &str, id: u32) -> String {
    const VALUE: u8 = 1;
    let bytes = [VALUE, 1]
        .iter()
        .chain(id.to_be_bytes().iter())
        .copied()
        .collect();

    convert_key(table_name, &Key::Bytea(bytes))
}

// Key format: table_name/,0,1,2,3,4

pub(crate) fn convert_key(table_name: &str, key: &Key) -> String {
    let key: Vec<_> = key.to_cmp_be_bytes().iter().map(u8::to_string).collect();
    format!("{}/{}", table_name, key.join(","))
}

pub(crate) fn retrieve_key(table_name: &str, key: &str) -> Result<Key> {
    let key = key
        .strip_prefix(&format!("{}/", table_name))
        .ok_or_else(|| StorageError::KeyParseError(key.to_owned()))?
        .split(',')
        .map(|s| s.parse::<u8>())
        .collect::<std::result::Result<_, _>>()
        .map_err(|_| StorageError::KeyParseError(key.to_owned()))?;

    Ok(Key::Bytea(key))
}
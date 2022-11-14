use {
    super::IndexeddbStorage,
    async_trait::async_trait,
    gluesql_core::{
        ast::ColumnDef,
        result::{Error, MutResult},
        store::AlterTable,
    },
};

#[async_trait(?Send)]
impl AlterTable for IndexeddbStorage {
    async fn rename_schema(self, _table_name: &str, _new_table_name: &str) -> MutResult<Self, ()> {
        Err((
            self,
            Error::StorageMsg("[IndexeddbStorage] AlterTable is not supported".to_owned()),
        ))
    }

    async fn rename_column(
        self,
        _table_name: &str,
        _old_column_name: &str,
        _new_column_name: &str,
    ) -> MutResult<Self, ()> {
        Err((
            self,
            Error::StorageMsg("[IndexeddbStorage] AlterTable is not supported".to_owned()),
        ))
    }

    async fn add_column(self, _table_name: &str, _column_def: &ColumnDef) -> MutResult<Self, ()> {
        Err((
            self,
            Error::StorageMsg("[IndexeddbStorage] AlterTable is not supported".to_owned()),
        ))
    }

    async fn drop_column(
        self,
        _table_name: &str,
        _column_name: &str,
        _if_exists: bool,
    ) -> MutResult<Self, ()> {
        Err((
            self,
            Error::StorageMsg("[IndexeddbStorage] AlterTable is not supported".to_owned()),
        ))
    }
}

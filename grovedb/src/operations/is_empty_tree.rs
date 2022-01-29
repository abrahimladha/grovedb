use std::collections::HashMap;
use merk::Merk;
use storage::{rocksdb_storage::OptimisticTransactionDBTransaction, RawIterator};
use storage::rocksdb_storage::PrefixedRocksDbStorage;

use crate::{Error, GroveDb};

impl GroveDb {
    pub fn is_empty_tree(
        &self,
        path: &[&[u8]],
        transaction: Option<&OptimisticTransactionDBTransaction>,
    ) -> Result<bool, Error> {
        let subtrees = match transaction {
            None => &self.subtrees,
            Some(_) => &self.temp_subtrees,
        };

        let merk = subtrees
            .get(&Self::compress_subtree_key(path, None))
            .ok_or(Error::InvalidPath("no subtree found under that path"))?;

        Ok(merk.is_empty_tree())
    }
}

use database::{Database, PrefixDB};

use crate::{
    types::kv::{mutable::KVStoreMut, KVStore, KVStoreBackend},
    QueryableMultiKVStore, StoreKey, TransactionalMultiKVStore,
};

use super::{commit::CommitMultiStore, MultiStore};

#[derive(Debug)]
pub struct MultiStoreMut<'a, DB, SK>(pub(crate) &'a mut CommitMultiStore<DB, SK>);

impl<DB, SK> MultiStoreMut<'_, DB, SK> {
    pub fn to_immutable(&self) -> MultiStore<'_, DB, SK> {
        MultiStore(super::MultiStoreBackend::Commit(self.0))
    }
}

impl<'a, DB: Database, SK: StoreKey> QueryableMultiKVStore<PrefixDB<DB>, SK>
    for MultiStoreMut<'a, DB, SK>
{
    fn kv_store(&self, store_key: &SK) -> KVStore<'_, PrefixDB<DB>> {
        KVStore(KVStoreBackend::Commit(self.0.kv_store(store_key)))
    }

    fn head_version(&self) -> u32 {
        self.0.head_version
    }

    fn head_commit_hash(&self) -> [u8; 32] {
        self.0.head_commit_hash
    }
}

impl<DB: Database, SK: StoreKey> TransactionalMultiKVStore<DB, SK> for MultiStoreMut<'_, DB, SK> {
    fn kv_store_mut(&mut self, store_key: &SK) -> KVStoreMut<'_, PrefixDB<DB>> {
        KVStoreMut(self.0.kv_store_mut(store_key))
    }

    fn tx_cache_to_block(&mut self) {
        self.0.tx_cache_to_block()
    }

    fn tx_caches_clear(&mut self) {
        self.0.tx_caches_clear()
    }
}

impl<'a, DB, SK> From<&'a mut CommitMultiStore<DB, SK>> for MultiStoreMut<'a, DB, SK> {
    fn from(value: &'a mut CommitMultiStore<DB, SK>) -> Self {
        MultiStoreMut(value)
    }
}

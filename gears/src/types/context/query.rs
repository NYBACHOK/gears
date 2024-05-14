use database::{Database, PrefixDB};

use store_crate::types::kv::immutable::KVStore;
use store_crate::types::multi::immutable::MultiStore;
use store_crate::types::query::versioned::VersionedQueryMultiStore;
use store_crate::QueryableMultiKVStore;
use store_crate::{error::StoreError, StoreKey};
use tendermint::types::chain_id::ChainId;

use super::QueryableContext;

pub struct QueryContext<'a, DB, SK> {
    multi_store: VersionedQueryMultiStore<'a, DB, SK>,
    pub(crate) height: u64,
    pub(crate) chain_id: ChainId,
}

impl<'a, DB: Database, SK: StoreKey> QueryContext<'a, DB, SK> {
    pub fn new(
        multi_store: VersionedQueryMultiStore<'a, DB, SK>,
        version: u32,
        // chain_id: ChainId,
    ) -> Result<Self, StoreError> {
        Ok(QueryContext {
            multi_store,
            height: version as u64, // TODO:
            chain_id: ChainId::new("todo-900").expect("default should be valid"),
        })
    }
}

impl<DB: Database, SK: StoreKey> QueryableContext<DB, SK> for QueryContext<'_, DB, SK> {
    fn kv_store(&self, store_key: &SK) -> KVStore<'_, PrefixDB<DB>> {
        self.multi_store.kv_store(store_key)
    }

    fn multi_store(&self) -> MultiStore<'_, DB, SK> {
        MultiStore::from(&self.multi_store)
    }

    fn height(&self) -> u64 {
        self.height
    }

    fn chain_id(&self) -> &ChainId {
        &self.chain_id
    }
}

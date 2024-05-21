use crate::{
    error::AppError,
    signing::renderer::value_renderer::ValueRenderer,
    types::{
        context::{block::BlockContext, init::InitContext, query::QueryContext, tx::TxContext},
        tx::{raw::TxWithRaw, TxMessage},
    },
};
use database::Database;
use serde::de::DeserializeOwned;
use store_crate::StoreKey;
use tendermint::types::{
    proto::validator::ValidatorUpdate,
    request::{begin_block::RequestBeginBlock, end_block::RequestEndBlock, query::RequestQuery},
};

pub trait AnteHandlerTrait<SK: StoreKey>: Clone + Send + Sync + 'static {
    fn run<DB: Database, M: TxMessage + ValueRenderer>(
        &self,
        ctx: &mut TxContext<'_, DB, SK>,
        tx: &TxWithRaw<M>,
    ) -> Result<(), AppError>;
}

pub trait ABCIHandler<
    M: TxMessage,
    SK: StoreKey,
    G: DeserializeOwned + Clone + Send + Sync + 'static,
>: Clone + Send + Sync + 'static
{
    fn run_ante_checks<DB: Database>(
        &self,
        ctx: &mut TxContext<'_, DB, SK>,
        tx: &TxWithRaw<M>,
    ) -> Result<(), AppError>;

    fn tx<DB: Database + Sync + Send>(
        &self,
        ctx: &mut TxContext<'_, DB, SK>,
        msg: &M,
    ) -> Result<(), AppError>;

    #[allow(unused_variables)]
    fn begin_block<'a, DB: Database>(
        &self,
        ctx: &mut BlockContext<'_, DB, SK>,
        request: RequestBeginBlock,
    ) {
    }

    #[allow(unused_variables)]
    fn end_block<'a, DB: Database>(
        &self,
        ctx: &mut BlockContext<'_, DB, SK>,
        request: RequestEndBlock,
    ) -> Vec<ValidatorUpdate> {
        Vec::new()
    }

    fn init_genesis<DB: Database>(&self, ctx: &mut InitContext<'_, DB, SK>, genesis: G);

    fn query<DB: Database + Send + Sync>(
        &self,
        ctx: &QueryContext<DB, SK>,
        query: RequestQuery,
    ) -> Result<bytes::Bytes, AppError>;
}
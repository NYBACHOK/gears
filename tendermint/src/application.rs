use tendermint_abci::Application;

use crate::types::{
    request::{
        begin_block::RequestBeginBlock,
        check_tx::RequestCheckTx,
        deliver_tx::RequestDeliverTx,
        echo::RequestEcho,
        end_block::RequestEndBlock,
        info::RequestInfo,
        init_chain::RequestInitChain,
        query::RequestQuery,
        snapshot::{RequestApplySnapshotChunk, RequestLoadSnapshotChunk, RequestOfferSnapshot},
    },
    response::{
        begin_block::ResponseBeginBlock,
        check_tx::ResponseCheckTx,
        deliver_tx::ResponseDeliverTx,
        echo::ResponseEcho,
        end_block::ResponseEndBlock,
        info::ResponseInfo,
        init_chain::ResponseInitChain,
        query::ResponseQuery,
        snapshot::{
            ResponseApplySnapshotChunk, ResponseListSnapshots, ResponseLoadSnapshotChunk,
            ResponseOfferSnapshot,
        },
        ResponseCommit, ResponseFlush,
    },
};
/// An ABCI application.
///
/// Applications are `Send` + `Clone` + `'static` because they are cloned for
/// each incoming connection to the ABCI [`Server`]. It is up to the
/// application developer to manage shared state between these clones of their
/// application.
///
/// [`Server`]: crate::Server
pub trait ABCIApplication: Send + Clone + 'static {
    /// Echo back the same message as provided in the request.
    fn echo(&self, request: RequestEcho) -> ResponseEcho {
        ResponseEcho {
            message: request.message,
        }
    }

    /// Provide information about the ABCI application.
    fn info(&self, _request: RequestInfo) -> ResponseInfo {
        Default::default()
    }

    /// Called once upon genesis.
    fn init_chain(&self, _request: RequestInitChain) -> ResponseInitChain {
        Default::default()
    }

    /// Query the application for data at the current or past height.
    fn query(&self, _request: RequestQuery) -> ResponseQuery {
        Default::default()
    }

    /// Check the given transaction before putting it into the local mempool.
    fn check_tx(&self, _request: RequestCheckTx) -> ResponseCheckTx {
        Default::default()
    }

    /// Signals the beginning of a new block, prior to any `DeliverTx` calls.
    fn begin_block(&self, _request: RequestBeginBlock) -> ResponseBeginBlock {
        Default::default()
    }

    /// Apply a transaction to the application's state.
    fn deliver_tx(&self, _request: RequestDeliverTx) -> ResponseDeliverTx {
        Default::default()
    }

    /// Signals the end of a block.
    fn end_block(&self, _request: RequestEndBlock) -> ResponseEndBlock {
        Default::default()
    }

    /// Signals that messages queued on the client should be flushed to the server.
    fn flush(&self) -> ResponseFlush {
        ResponseFlush {}
    }

    /// Commit the current state at the current height.
    fn commit(&self) -> ResponseCommit {
        Default::default()
    }

    /// Used during state sync to discover available snapshots on peers.
    fn list_snapshots(&self) -> ResponseListSnapshots {
        Default::default()
    }

    /// Called when bootstrapping the node using state sync.
    fn offer_snapshot(&self, _request: RequestOfferSnapshot) -> ResponseOfferSnapshot {
        Default::default()
    }

    /// Used during state sync to retrieve chunks of snapshots from peers.
    fn load_snapshot_chunk(&self, _request: RequestLoadSnapshotChunk) -> ResponseLoadSnapshotChunk {
        Default::default()
    }

    /// Apply the given snapshot chunk to the application's state.
    fn apply_snapshot_chunk(
        &self,
        _request: RequestApplySnapshotChunk,
    ) -> ResponseApplySnapshotChunk {
        Default::default()
    }
}

#[derive(Debug, Clone)]
pub struct ABCI<T: ABCIApplication> {
    handler: T,
}

impl<T: ABCIApplication> From<T> for ABCI<T> {
    fn from(handler: T) -> Self {
        Self { handler }
    }
}

impl<T: ABCIApplication> Application for ABCI<T> {
    fn echo(
        &self,
        request: tendermint_proto::abci::RequestEcho,
    ) -> tendermint_proto::abci::ResponseEcho {
        T::echo(&self.handler, request.into()).into()
    }

    fn info(
        &self,
        request: tendermint_proto::abci::RequestInfo,
    ) -> tendermint_proto::abci::ResponseInfo {
        T::info(&self.handler, request.into()).into()
    }

    fn init_chain(
        &self,
        request: tendermint_proto::abci::RequestInitChain,
    ) -> tendermint_proto::abci::ResponseInitChain {
        T::init_chain(&self.handler, request.into()).into()
    }

    fn query(
        &self,
        request: tendermint_proto::abci::RequestQuery,
    ) -> tendermint_proto::abci::ResponseQuery {
        T::query(&self.handler, request.into()).into()
    }

    fn check_tx(
        &self,
        request: tendermint_proto::abci::RequestCheckTx,
    ) -> tendermint_proto::abci::ResponseCheckTx {
        T::check_tx(&self.handler, request.into()).into()
    }

    fn begin_block(
        &self,
        request: tendermint_proto::abci::RequestBeginBlock,
    ) -> tendermint_proto::abci::ResponseBeginBlock {
        T::begin_block(&self.handler, request.into()).into()
    }

    fn deliver_tx(
        &self,
        request: tendermint_proto::abci::RequestDeliverTx,
    ) -> tendermint_proto::abci::ResponseDeliverTx {
        T::deliver_tx(&self.handler, request.into()).into()
    }

    fn end_block(
        &self,
        request: tendermint_proto::abci::RequestEndBlock,
    ) -> tendermint_proto::abci::ResponseEndBlock {
        T::end_block(&self.handler, request.into()).into()
    }

    fn flush(&self) -> tendermint_proto::abci::ResponseFlush {
        T::flush(&self.handler).into()
    }

    fn commit(&self) -> tendermint_proto::abci::ResponseCommit {
        T::commit(&self.handler).into()
    }

    fn list_snapshots(&self) -> tendermint_proto::abci::ResponseListSnapshots {
        T::list_snapshots(&self.handler).into()
    }

    fn offer_snapshot(
        &self,
        request: tendermint_proto::abci::RequestOfferSnapshot,
    ) -> tendermint_proto::abci::ResponseOfferSnapshot {
        T::offer_snapshot(&self.handler, request.into()).into()
    }

    fn load_snapshot_chunk(
        &self,
        request: tendermint_proto::abci::RequestLoadSnapshotChunk,
    ) -> tendermint_proto::abci::ResponseLoadSnapshotChunk {
        T::load_snapshot_chunk(&self.handler, request.into()).into()
    }

    fn apply_snapshot_chunk(
        &self,
        request: tendermint_proto::abci::RequestApplySnapshotChunk,
    ) -> tendermint_proto::abci::ResponseApplySnapshotChunk {
        T::apply_snapshot_chunk(&self.handler, request.into()).into()
    }
}
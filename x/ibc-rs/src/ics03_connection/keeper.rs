use gears::context::TransactionalContext;
use gears::{
    context::init::InitContext,
    params::ParamsSubspaceKey,
    store::{database::Database, StoreKey},
};

use super::{params::ConnectionParamsKeeper, GenesisState};

const KEY_NEXT_CONNECTION_SEQUENCE: &[u8; 22] = b"nextConnectionSequence";

#[derive(Debug, Clone)]
pub struct Keeper<SK, PSK> {
    store_key: SK,
    connection_params_keeper: ConnectionParamsKeeper<PSK>,
}

impl<SK: StoreKey, PSK: ParamsSubspaceKey> Keeper<SK, PSK> {
    pub fn new(store_key: SK, params_subspace_key: PSK) -> Self {
        let connection_params_keeper = ConnectionParamsKeeper {
            params_subspace_key,
        };
        Self {
            store_key,
            connection_params_keeper,
        }
    }

    pub fn init_genesis<DB: Database>(
        &self,
        ctx: &mut InitContext<'_, DB, SK>,
        genesis: GenesisState,
    ) {
        // TODO: the following lines(from ibc-go) have not been implemented yet:
        // for _, connection := range gs.Connections {
        //     conn := types.NewConnectionEnd(connection.State, connection.ClientId, connection.Counterparty, connection.Versions, connection.DelayPeriod)
        //     k.SetConnection(ctx, connection.Id, conn)
        // }
        // for _, connPaths := range gs.ClientConnectionPaths {
        //     k.SetClientConnectionPaths(ctx, connPaths.ClientId, connPaths.Paths)
        // }

        self.set_next_connection_sequence(ctx, genesis.next_connection_sequence);
        self.connection_params_keeper
            .set(ctx, genesis.params.clone());
    }

    pub fn set_next_connection_sequence<DB: Database>(
        &self,
        ctx: &mut InitContext<'_, DB, SK>,
        sequence: u64,
    ) {
        let mut ibc_store = ctx.kv_store_mut(&self.store_key);
        ibc_store.set(
            KEY_NEXT_CONNECTION_SEQUENCE.to_owned(),
            sequence.to_be_bytes(),
        );
    }
}

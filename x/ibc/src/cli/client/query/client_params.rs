use clap::Args;
use gears::{client::query::run_query, types::context::query_context::QueryContext};
use prost::Message;
use proto_messages::cosmos::ibc::{
    query::QueryClientParamsResponse,
    types::core::{
        client::context::types::proto::v1::QueryClientParamsResponse as RawQueryClientParamsResponse,
        connection::proto::v1::QueryClientConnectionsRequest,
    },
};
use tendermint::informal::block::Height;

#[derive(Args, Debug, Clone)]
pub struct CliClientParams {
    client_id: String,
}

#[allow(dead_code)]
pub(super) fn query_command_handler<DB, SK>(
    _ctx: &QueryContext<'_, DB, SK>,
    args: CliClientParams,
    node: &str,
    height: Option<Height>,
) -> anyhow::Result<String> {
    let query = QueryClientConnectionsRequest {
        client_id: args.client_id,
    };

    let result = run_query::<QueryClientParamsResponse, RawQueryClientParamsResponse>(
        query.encode_to_vec(),
        "/cosmos.bank.v1beta1.Query/AllBalances".to_owned(),
        node,
        height,
    )?;

    let result = serde_json::to_string_pretty(&result)?;

    Ok(result)
}

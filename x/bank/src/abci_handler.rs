use auth::ante::BankKeeper;
use gears::ibc::errors::Error as IbcError;
use gears::store::database::Database;
use gears::tendermint::types::proto::Protobuf;
use gears::tendermint::types::request::query::RequestQuery;
use gears::types::context::init_context::InitContext;
use gears::types::context::query_context::QueryContext;
use gears::types::context::tx_context::TxContext;
use gears::{error::AppError, x::params::ParamsSubspaceKey};
// use proto_messages::cosmos::bank::v1beta1::{
//     QueryAllBalancesRequest, QueryBalanceRequest, QueryDenomMetadataRequest,
//     QueryDenomMetadataResponse, QueryTotalSupplyResponse,
// };
// use proto_messages::cosmos::ibc::protobuf::Protobuf;
use gears::store::StoreKey;

use crate::types::query::{
    QueryAllBalancesRequest, QueryBalanceRequest, QueryDenomMetadataRequest,
    QueryDenomMetadataResponse, QueryTotalSupplyResponse,
};
use crate::{GenesisState, Keeper, Message};

#[derive(Debug, Clone)]
pub struct ABCIHandler<SK: StoreKey, PSK: ParamsSubspaceKey> {
    keeper: Keeper<SK, PSK>,
}

impl<'a, SK: StoreKey, PSK: ParamsSubspaceKey> ABCIHandler<SK, PSK> {
    pub fn new(keeper: Keeper<SK, PSK>) -> Self {
        ABCIHandler { keeper }
    }

    pub fn tx<DB: Database>(
        &self,
        ctx: &mut TxContext<'_, DB, SK>,
        msg: &Message,
    ) -> Result<(), AppError> {
        match msg {
            Message::Send(msg_send) => self
                .keeper
                .send_coins_from_account_to_account(ctx, msg_send),
        }
    }

    pub fn query<DB: Database>(
        &self,
        ctx: &QueryContext<'a, DB, SK>,
        query: RequestQuery,
    ) -> std::result::Result<bytes::Bytes, AppError> {
        match query.path.as_str() {
            "/cosmos.bank.v1beta1.Query/AllBalances" => {
                let req = QueryAllBalancesRequest::decode(query.data)
                    .map_err(|e| IbcError::DecodeProtobuf(e.to_string()))?;

                Ok(self
                    .keeper
                    .query_all_balances(ctx, req)
                    .encode_vec()
                    .expect("msg")
                    .into()) // TODO:NOW
            }
            "/cosmos.bank.v1beta1.Query/TotalSupply" => Ok(QueryTotalSupplyResponse {
                supply: self.keeper.get_paginated_total_supply(ctx),
                pagination: None,
            }
            .encode_vec().expect("msg")
            .into()), // TODO:NOW
            "/cosmos.bank.v1beta1.Query/Balance" => {
                let req = QueryBalanceRequest::decode(query.data)
                    .map_err(|e| IbcError::DecodeProtobuf(e.to_string()))?; 

                Ok(self.keeper.query_balance(ctx, req).encode_vec().expect("msg").into()) // TODO:NOW
            }
            "/cosmos.bank.v1beta1.Query/DenomsMetadata" => {
                Ok(self.keeper.query_denoms_metadata(ctx).encode_vec().expect("msg").into()) // TODO:NOW
            }
            "/cosmos.bank.v1beta1.Query/DenomMetadata" => {
                let req = QueryDenomMetadataRequest::decode(query.data)
                    .map_err(|e| IbcError::DecodeProtobuf(e.to_string()))?; // TODO:NOW
                let metadata = self.keeper.get_denom_metadata(ctx, &req.denom);
                Ok(QueryDenomMetadataResponse { metadata }.encode_vec().expect("msg").into()) // TODO:NOW
            }
            _ => Err(AppError::InvalidRequest("query path not found".into())),
        }
    }

    pub fn genesis<DB: Database>(&self, ctx: &mut InitContext<'_, DB, SK>, genesis: GenesisState) {
        self.keeper.init_genesis(ctx, genesis)
    }
}

use ibc_proto::protobuf::Protobuf;
use proto_types::AccAddress;
use std::hash::Hash;
use tendermint_abci::Application;

use axum::{
    body::Body,
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use gears::{
    baseapp::{
        ante::{AuthKeeper, BankKeeper},
        BaseApp, Handler,
    },
    client::rest::{error::Error, Pagination},
    x::params::ParamsSubspaceKey,
};
use proto_messages::cosmos::{
    bank::v1beta1::{
        QueryAllBalancesRequest, QueryAllBalancesResponse, QueryBalanceRequest,
        QueryBalanceResponse, QueryTotalSupplyResponse,
    },
    tx::v1beta1::Message,
};
use serde::{de::DeserializeOwned, Deserialize};
use store::StoreKey;
use strum::IntoEnumIterator;
use tendermint_proto::abci::RequestQuery;

/// Gets the total supply of every denom
pub async fn supply<
    SK: Hash + Eq + IntoEnumIterator + StoreKey + Clone + Send + Sync + 'static,
    PSK: ParamsSubspaceKey + Clone + Send + Sync + 'static,
    M: Message,
    BK: BankKeeper<SK> + Clone + Send + Sync + 'static,
    AK: AuthKeeper<SK> + Clone + Send + Sync + 'static,
    H: Handler<M, SK, G> + 'static,
    G: DeserializeOwned + Clone + Send + Sync + 'static,
>(
    State(app): State<BaseApp<SK, PSK, M, BK, AK, H, G>>,
) -> Result<Json<QueryTotalSupplyResponse>, Error> {
    let request = RequestQuery {
        data: Default::default(),
        path: "/cosmos.bank.v1beta1.Query/TotalSupply".into(),
        height: 0,
        prove: false,
    };

    let response = app.query(request);

    Ok(Json(
        QueryTotalSupplyResponse::decode(response.value)
            .expect("should be a valid QueryTotalSupplyResponse"),
    ))
}

/// Get all balances for a given address
pub async fn get_balances<
    SK: Hash + Eq + IntoEnumIterator + StoreKey + Clone + Send + Sync + 'static,
    PSK: ParamsSubspaceKey + Clone + Send + Sync + 'static,
    M: Message,
    BK: BankKeeper<SK> + Clone + Send + Sync + 'static,
    AK: AuthKeeper<SK> + Clone + Send + Sync + 'static,
    H: Handler<M, SK, G> + 'static,
    G: DeserializeOwned + Clone + Send + Sync + 'static,
>(
    Path(address): Path<AccAddress>,
    _pagination: Query<Pagination>,
    State(app): State<BaseApp<SK, PSK, M, BK, AK, H, G>>,
) -> Result<Json<QueryAllBalancesResponse>, Error> {
    let req = QueryAllBalancesRequest {
        address,
        pagination: None,
    };

    let request = RequestQuery {
        data: req.encode_vec().into(),
        path: "/cosmos.bank.v1beta1.Query/AllBalances".into(),
        height: 0,
        prove: false,
    };

    let response = app.query(request);

    Ok(Json(
        QueryAllBalancesResponse::decode(response.value)
            .expect("should be a valid QueryAllBalancesResponse"),
    ))
}

#[derive(Deserialize)]
pub struct RawDenom {
    denom: String,
}

// TODO: returns {"balance":null} if balance is zero, is this expected?
/// Get balance for a given address and denom
//#[get("/cosmos/bank/v1beta1/balances/<addr>/by_denom?<denom>")]
pub async fn get_balances_by_denom<
    SK: Hash + Eq + IntoEnumIterator + StoreKey + Clone + Send + Sync + 'static,
    PSK: ParamsSubspaceKey + Clone + Send + Sync + 'static,
    M: Message,
    BK: BankKeeper<SK> + Clone + Send + Sync + 'static,
    AK: AuthKeeper<SK> + Clone + Send + Sync + 'static,
    H: Handler<M, SK, G> + 'static,
    G: DeserializeOwned + Clone + Send + Sync + 'static,
>(
    Path(address): Path<AccAddress>,
    denom: Query<RawDenom>,
    State(app): State<BaseApp<SK, PSK, M, BK, AK, H, G>>,
) -> Result<Json<QueryBalanceResponse>, Error> {
    let req = QueryBalanceRequest {
        address,
        denom: String::from(denom.0.denom)
            .try_into()
            .map_err(|e: proto_types::Error| Error::bad_request(e.to_string()))?,
    };

    let request = RequestQuery {
        data: req.encode_vec().into(),
        path: "/cosmos.bank.v1beta1.Query/Balance".into(),
        height: 0,
        prove: false,
    };

    let response = app.query(request);

    Ok(Json(
        QueryBalanceResponse::decode(response.value)
            .expect("should be a valid QueryBalanceResponse"),
    ))
}

pub fn get_router<
    SK: Hash + Eq + IntoEnumIterator + StoreKey + Clone + Send + Sync + 'static,
    PSK: ParamsSubspaceKey + Clone + Send + Sync + 'static,
    M: Message,
    BK: BankKeeper<SK> + Clone + Send + Sync + 'static,
    AK: AuthKeeper<SK> + Clone + Send + Sync + 'static,
    H: Handler<M, SK, G> + 'static,
    G: DeserializeOwned + Clone + Send + Sync + 'static,
>() -> Router<BaseApp<SK, PSK, M, BK, AK, H, G>, Body> {
    Router::new()
        .route("/v1beta1/supply", get(supply))
        .route("/v1beta1/balances/:address", get(get_balances))
        .route(
            "/v1beta1/balances/:address/by_denom",
            get(get_balances_by_denom),
        )
}

use gears::{
    baseapp::{QueryRequest, QueryResponse},
    error::IBC_ENCODE_UNWRAP,
    tendermint::types::proto::Protobuf,
    types::query::Query,
};
use request::{
    QueryAllParamsRequest, QueryDepositRequest, QueryDepositsRequest, QueryParamsRequest,
    QueryProposalRequest, QueryProposalsRequest, QueryProposerRequest, QueryTallyResultRequest,
    QueryVoteRequest, QueryVotesRequest,
};
use response::{
    QueryAllParamsResponse, QueryDepositResponse, QueryDepositsResponse, QueryParamsResponse,
    QueryProposalResponse, QueryProposalsResponse, QueryProposerResponse, QueryTallyResultResponse,
    QueryVoteResponse, QueryVotesResponse,
};
use serde::{Deserialize, Serialize};

pub mod request;
pub mod response;

#[derive(Debug, Clone)]
pub enum GovQuery {
    Deposit(QueryDepositRequest),
    Deposits(QueryDepositsRequest),
    Params(QueryParamsRequest),
    AllParams(QueryAllParamsRequest),
    Proposal(QueryProposalRequest),
    Proposals(QueryProposalsRequest),
    Tally(QueryTallyResultRequest),
    Vote(QueryVoteRequest),
    Votes(QueryVotesRequest),
    Proposer(QueryProposerRequest),
}

impl Query for GovQuery {
    fn query_url(&self) -> &'static str {
        match self {
            GovQuery::Deposit(_) => "/cosmos.gov.v1beta1.Query/Deposit",
            GovQuery::Deposits(_) => "/cosmos.gov.v1beta1.Query/Deposits",
            GovQuery::Params(_) => "/cosmos.gov.v1beta1.Query/Param",
            GovQuery::AllParams(_) => "/cosmos.gov.v1beta1.Query/Params",
            GovQuery::Proposal(_) => "/cosmos.gov.v1beta1.Query/Proposal",
            GovQuery::Proposals(_) => "/cosmos.gov.v1beta1.Query/Proposals",
            GovQuery::Tally(_) => "/cosmos.gov.v1beta1.Query/Tally",
            GovQuery::Vote(_) => "/cosmos.gov.v1beta1.Query/Vote",
            GovQuery::Votes(_) => "/cosmos.gov.v1beta1.Query/Votes",
            GovQuery::Proposer(_) => "/cosmos.gov.v1beta1.Query/Proposer",
        }
    }

    fn into_bytes(self) -> Vec<u8> {
        match self {
            GovQuery::Deposit(q) => q.encode_vec().expect(IBC_ENCODE_UNWRAP),
            GovQuery::Deposits(q) => q.encode_vec().expect(IBC_ENCODE_UNWRAP),
            GovQuery::Params(q) => q.encode_vec().expect(IBC_ENCODE_UNWRAP),
            GovQuery::AllParams(q) => q.encode_vec().expect(IBC_ENCODE_UNWRAP),
            GovQuery::Proposal(q) => q.encode_vec().expect(IBC_ENCODE_UNWRAP),
            GovQuery::Proposals(q) => q.encode_vec().expect(IBC_ENCODE_UNWRAP),
            GovQuery::Tally(q) => q.encode_vec().expect(IBC_ENCODE_UNWRAP),
            GovQuery::Vote(q) => q.encode_vec().expect(IBC_ENCODE_UNWRAP),
            GovQuery::Votes(q) => q.encode_vec().expect(IBC_ENCODE_UNWRAP),
            GovQuery::Proposer(q) => q.encode_vec().expect(IBC_ENCODE_UNWRAP),
        }
    }
}

impl QueryRequest for GovQuery {
    fn height(&self) -> u32 {
        todo!()
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum GovQueryResponse {
    Deposit(QueryDepositResponse),
    Deposits(QueryDepositsResponse),
    Params(QueryParamsResponse),
    AllParams(QueryAllParamsResponse),
    Proposal(QueryProposalResponse),
    Proposals(QueryProposalsResponse),
    Tally(QueryTallyResultResponse),
    Vote(QueryVoteResponse),
    Votes(QueryVotesResponse),
    Proposer(QueryProposerResponse),
}

impl QueryResponse for GovQueryResponse {}

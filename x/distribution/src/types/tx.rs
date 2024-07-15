use gears::{
    core::Protobuf,
    types::address::{AccAddress, AddressError, ValAddress},
};
use prost::Message;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Message)]
pub struct MsgWithdrawDelegatorRewardRaw {
    #[prost(bytes, tag = "1")]
    pub validator_address: Vec<u8>,
    #[prost(bytes, tag = "2")]
    pub delegator_address: Vec<u8>,
    #[prost(bool, tag = "3")]
    pub withdraw_commission: bool,
}

impl From<MsgWithdrawDelegatorReward> for MsgWithdrawDelegatorRewardRaw {
    fn from(
        MsgWithdrawDelegatorReward {
            validator_address,
            delegator_address,
            withdraw_commission,
        }: MsgWithdrawDelegatorReward,
    ) -> Self {
        Self {
            validator_address: validator_address.into(),
            delegator_address: delegator_address.into(),
            withdraw_commission,
        }
    }
}

/// MsgWithdrawDelegatorReward represents delegation withdrawal to a delegator
/// from a single validator.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MsgWithdrawDelegatorReward {
    pub validator_address: ValAddress,
    pub delegator_address: AccAddress,
    pub withdraw_commission: bool,
}

impl TryFrom<MsgWithdrawDelegatorRewardRaw> for MsgWithdrawDelegatorReward {
    type Error = AddressError;

    fn try_from(
        MsgWithdrawDelegatorRewardRaw {
            validator_address,
            delegator_address,
            withdraw_commission,
        }: MsgWithdrawDelegatorRewardRaw,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            validator_address: ValAddress::try_from(validator_address)?,
            delegator_address: AccAddress::try_from(delegator_address)?,
            withdraw_commission,
        })
    }
}

impl Protobuf<MsgWithdrawDelegatorRewardRaw> for MsgWithdrawDelegatorReward {}

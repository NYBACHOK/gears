use std::{fs::File, io::Read};

use clap::Args;
use proto_messages::cosmos::ibc::{
    tx::MsgUpgradeClient,
    types::{
        core::{commitment::CommitmentProofBytes, host::identifiers::ClientId},
        tendermint::{consensus_state::WrappedConsensusState, WrappedTendermintClientState},
    },
};

use crate::types::Signer;

/// upgrade an IBC client
#[derive(Args, Debug, Clone)]
pub struct CliUpgradeClient {
    pub client_id: ClientId,
    pub upgraded_client_state: String,
    pub upgraded_consensus_state: String,
    pub proof_upgrade_client: String,
    pub proof_upgrade_consensus_state: String,
    pub signer: Signer,
}

pub(super) fn tx_command_handler(msg: CliUpgradeClient) -> anyhow::Result<crate::message::Message> {
    let CliUpgradeClient {
        client_id,
        upgraded_client_state,
        upgraded_consensus_state,
        proof_upgrade_client,
        proof_upgrade_consensus_state,
        signer,
    } = msg;

    let mut buffer = Vec::<u8>::new();

    let upgraded_client_state_res =
        serde_json::from_str::<WrappedTendermintClientState>(&upgraded_client_state);
    let upgraded_client_state = if let Ok(upgraded_client_state) = upgraded_client_state_res {
        upgraded_client_state
    } else {
        File::open(upgraded_client_state)?.read_to_end(&mut buffer)?;
        serde_json::from_slice(&buffer)?
    };

    let upgraded_consensus_state_res =
        serde_json::from_str::<WrappedConsensusState>(&upgraded_consensus_state);
    let upgraded_consensus_state =
        if let Ok(upgraded_consensus_state) = upgraded_consensus_state_res {
            upgraded_consensus_state
        } else {
            File::open(upgraded_consensus_state)?.read_to_end(&mut buffer)?;
            serde_json::from_slice(&buffer)?
        };

    let raw_msg = MsgUpgradeClient {
        client_id,
        upgraded_client_state,
        upgraded_consensus_state,
        proof_upgrade_client: CommitmentProofBytes::try_from(proof_upgrade_client.into_bytes())?,
        proof_upgrade_consensus_state: CommitmentProofBytes::try_from(
            proof_upgrade_consensus_state.into_bytes(),
        )?,
        signer: proto_messages::cosmos::ibc::types::primitives::Signer::from(signer.0),
    };

    Ok(crate::message::Message::ClientUpgrade(raw_msg))
}

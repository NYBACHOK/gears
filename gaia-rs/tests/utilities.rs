//! This modules should be added to test modules with `#[path = "./utilities.rs"]` as it contains gaia specific code and dedicated crate is bothersome.
#![allow(dead_code)]

use std::{path::PathBuf, time::Duration};

use gaia_rs::{
    abci_handler::GaiaABCIHandler, config::AppConfig, genesis::GenesisState,
    store_keys::GaiaParamsStoreKey, GaiaApplication, GaiaCore,
};
use gears::{
    application::node::NodeApplication,
    baseapp::genesis::{Genesis, GenesisError},
    commands::{
        client::keys::{keys, AddKeyCommand, KeyCommand, KeyringBackend},
        node::{
            run::{LogLevel, RunCommand},
            AppCommands,
        },
    },
    store::database::rocks::RocksDBBuilder,
    types::base::coins::UnsignedCoins,
};
use gears::{types::address::AccAddress, utils::TmpChild};

pub const TENDERMINT_PATH: &str = "./tests/assets";
pub const BIP39_MNEMONIC : &str = "race draft rival universe maid cheese steel logic crowd fork comic easy truth drift tomorrow eye buddy head time cash swing swift midnight borrow";

pub const NODE_URL_STR: &str = "http://localhost:26657/";

pub fn node_url() -> url::Url {
    NODE_URL_STR.try_into().expect("Default should be valid")
}

pub const ACC_ADDRESS: &str = "cosmos1syavy2npfyt9tcncdtsdzf7kny9lh777pahuux";

pub fn acc_address() -> AccAddress {
    AccAddress::from_bech32(ACC_ADDRESS).expect("Default Address should be valid")
}

/// Helper method to start gaia node and tendermint in tmp folder
pub fn run_gaia_and_tendermint(
    coins: u32,
) -> anyhow::Result<(TmpChild, std::thread::JoinHandle<()>)> {
    let tendermint = TmpChild::run_tendermint::<_, AppConfig>(
        TENDERMINT_PATH,
        &MockGenesis::default(),
        acc_address(),
        coins,
    )?;

    key_add(tendermint.to_path_buf(), KEY_NAME, BIP39_MNEMONIC)?;

    let home = tendermint.to_path_buf();
    let address = tendermint.proxy_addr().to_owned();
    let rest_addr = { std::net::TcpListener::bind("127.0.0.1:0")?.local_addr()? };

    let server_thread = std::thread::spawn(move || {
        let node = NodeApplication::<GaiaCore, _, _, _>::new(
            GaiaCore,
            RocksDBBuilder,
            GaiaABCIHandler::new,
            GaiaParamsStoreKey::BaseApp,
        );

        let cmd = RunCommand {
            home,
            address: Some(address),
            rest_listen_addr: Some(rest_addr),
            read_buf_size: 1048576,
            log_level: LogLevel::Off,
            min_gas_prices: Default::default(),
        };

        let _ = node.execute::<GaiaApplication>(AppCommands::Run(cmd));
    });

    std::thread::sleep(Duration::from_secs(10));

    Ok((tendermint, server_thread))
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct MockGenesis(pub GenesisState);

impl Genesis for MockGenesis {
    fn add_genesis_account(
        &mut self,
        address: AccAddress,
        coins: UnsignedCoins,
    ) -> Result<(), GenesisError> {
        self.0.add_genesis_account(address, coins)
    }
}

pub const KEY_NAME: &str = "alice";

pub fn key_add(home: impl Into<PathBuf>, name: &str, mnemonic: &str) -> anyhow::Result<()> {
    let cmd = AddKeyCommand {
        name: name.to_owned(),
        recover: true,
        home: home.into(),
        keyring_backend: KeyringBackend::Test,
        bip39_mnemonic: Some(mnemonic.to_owned()),
    };

    keys(KeyCommand::Add(cmd))?;

    Ok(())
}

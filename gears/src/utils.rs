use std::{
    net::{SocketAddr, TcpListener},
    path::{Path, PathBuf},
    process::Child,
    str::FromStr,
};

use crate::{
    baseapp::genesis::Genesis,
    commands::node::{
        genesis::{genesis_account_add, GenesisCommand},
        init::{init, InitCommand},
    },
    types::{
        address::AccAddress,
        base::{coin::UnsignedCoin, coins::Coins},
        denom::Denom,
    },
};
use anyhow::anyhow;
pub use assert_fs::TempDir;

use run_script::{IoOptions, ScriptOptions};
use tendermint::types::chain_id::ChainId;

/// Struct for process which lauched from tmp dir
#[derive(Debug)]
pub struct TmpChild {
    pub child: Child,
    pub tmp_dir: TempDir,
    rpc_addr: SocketAddr,
    node_addr: SocketAddr,
    proxy_addr: SocketAddr,
}

impl TmpChild {
    pub fn to_path_buf(&self) -> PathBuf {
        self.tmp_dir.to_path_buf()
    }

    /// tcp://0.0.0.0:26656
    pub fn node_addr(&self) -> &SocketAddr {
        &self.node_addr
    }

    /// tcp://127.0.0.1:26657
    pub fn rpc_addr(&self) -> &SocketAddr {
        &self.rpc_addr
    }

    /// tcp://127.0.0.1:26658
    pub fn proxy_addr(&self) -> &SocketAddr {
        &self.proxy_addr
    }
}

impl Drop for TmpChild {
    fn drop(&mut self) {
        // Stop child process before deletion of tmp dir
        while let Err(_) = self.child.kill() {
            std::thread::sleep(std::time::Duration::from_millis(100))
        }
    }
}

impl TmpChild {
    pub fn run_tendermint<G: Genesis, AC: crate::config::ApplicationConfig>(
        path_to_tendermint: &(impl AsRef<Path> + ?Sized),
        genesis: &G,
        address: AccAddress,
        coins: u32,
    ) -> anyhow::Result<Self> {
        let tmp_dir = TempDir::new()?;

        dircpy::CopyBuilder::new(path_to_tendermint, &tmp_dir)
            .overwrite(true)
            .run()?;

        let options = ScriptOptions {
            runner: None,
            runner_args: None,
            working_directory: Some(tmp_dir.to_path_buf()),
            input_redirection: IoOptions::Inherit,
            output_redirection: IoOptions::Pipe,
            exit_on_error: false,
            print_commands: false,
            env_vars: None,
        };

        let opt: InitCommand = InitCommand::former()
            .home(tmp_dir.to_path_buf())
            .chain_id(ChainId::from_str("test-chain")?)
            .moniker("test".to_owned())
            .form();

        init::<_, AC>(opt, genesis)?;

        let genesis_account_cmd = GenesisCommand {
            home: tmp_dir.to_path_buf(),
            address,
            coins: Coins::new(vec![UnsignedCoin {
                denom: Denom::from_str("uatom").expect("default denom should be valid"),
                amount: coins.into(),
            }])
            .expect("not empty"),
        };

        genesis_account_add::<G>(genesis_account_cmd)?;

        let (_code, _output, _error) = run_script::run(
            r#"
                tar -xf tendermint.tar.gz
                "#,
            &vec![],
            &options,
        )?; // TODO: make it work for windows too?

        let (rpc_addr, node_addr, proxy_addr) = three_random_adresses()?;
        let node_argument = format!("--p2p.laddr tcp://127.0.0.1:{}", node_addr.port());
        let rpc_argument = format!("--rpc.laddr tcp://127.0.0.1:{}", rpc_addr.port());
        let proxy_argument = format!("--proxy_app tcp://127.0.0.1:{}", proxy_addr.port());

        let script = format!(
            "./tendermint start --home {} {node_argument} {rpc_argument} {proxy_argument}",
            tmp_dir
                .to_str()
                .ok_or(anyhow!("failed to get path to tmp folder"))?
        );

        let child = run_script::spawn(&script, &vec![], &options)?;

        Ok(Self {
            child,
            tmp_dir,
            rpc_addr,
            node_addr,
            proxy_addr,
        })
    }
}

fn three_random_adresses() -> std::io::Result<(SocketAddr, SocketAddr, SocketAddr)> {
    let first = TcpListener::bind("127.0.0.1:0")?;
    let second = TcpListener::bind("127.0.0.1:0")?;
    let third = TcpListener::bind("127.0.0.1:0")?;

    Ok((
        first.local_addr()?,
        second.local_addr()?,
        third.local_addr()?,
    ))
}

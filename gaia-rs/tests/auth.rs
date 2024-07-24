use auth::{
    cli::query::{AccountCommand, AuthCommands, AuthQueryCli, AuthQueryResponse},
    query::QueryAccountResponse,
};
use gaia_rs::{
    client::{GaiaQueryCommands, WrappedGaiaQueryCommands},
    query::GaiaQueryResponse,
    GaiaCoreClient,
};
use gears::{
    commands::client::query::{run_query, QueryCommand},
    types::account::{Account, BaseAccount},
    types::address::AccAddress,
};

use utilities::run_gaia_and_tendermint;

#[path = "./utilities.rs"]
mod utilities;

#[test]
fn account_query() -> anyhow::Result<()> {
    let (tendermint, _server_thread) = run_gaia_and_tendermint(34)?;

    let acc_adress = AccAddress::from_bech32("cosmos1syavy2npfyt9tcncdtsdzf7kny9lh777pahuux")
        .expect("Valid value");

    let query = AccountCommand {
        address: acc_adress.clone(),
    };
 
    let cmd = QueryCommand {
        node: format!("http://localhost:{}/", tendermint.rpc_addr().port()).parse()?,
        height: None,
        inner: WrappedGaiaQueryCommands(GaiaQueryCommands::Auth(AuthQueryCli {
            command: AuthCommands::Account(query),
        })),
    };

    let result = run_query(cmd, &GaiaCoreClient)?;

    let expected = GaiaQueryResponse::Auth(AuthQueryResponse::Account(QueryAccountResponse {
        account: Some(Account::Base(BaseAccount {
            address: acc_adress,
            pub_key: None,
            account_number: 2,
            sequence: 0,
        })),
    }));

    assert_eq!(result, expected);

    Ok(())
}

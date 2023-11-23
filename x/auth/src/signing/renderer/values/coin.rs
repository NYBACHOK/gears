use std::ops::Div;

use database::RocksDB;
use gears::types::context::context::Context;
use num_bigint::ToBigUint;
use proto_messages::cosmos::{
    base::v1beta1::Coin,
    tx::v1beta1::{
        screen::{Content, Indent, Screen},
        tx_metadata::Metadata,
    },
};
use store::StoreKey;
use crate::signing::renderer::value_renderer::{DefaultPrimitiveRenderer, PrimitiveValueRenderer, ValueRenderer};

impl<DefaultValueRenderer, SK: StoreKey> ValueRenderer<DefaultValueRenderer, SK> for Coin {
    /// Format `Coin` into `Screen`.
    fn format(
        &self,
        ctx: &Context<'_, '_, RocksDB, SK>,
    ) -> Result<Vec<Screen>, Box<dyn std::error::Error>> {

        let Metadata {
            display,
            denom_units,
            ..
        } = ctx.metadata_get();

        let coin_exp = denom_units.iter().find(|this| this.denom == self.denom);
        let denom_exp = denom_units.iter().find(|this| this.denom.as_ref() == display);

        match (coin_exp, denom_exp) {
            (Some(coin_exp), Some(denom_exp)) => {
                let power = match coin_exp.exponent > denom_exp.exponent {
                    true => coin_exp.exponent - denom_exp.exponent,
                    false => denom_exp.exponent - coin_exp.exponent,
                };

                let disp_amount = self
                    .amount.clone()
                    .div( 10.to_biguint().expect( "Should be able to parse number").pow( power ));

                let formated_amount = DefaultPrimitiveRenderer::format( disp_amount );

                let screen = Screen {
                    title: "Amount".to_string(),
                    content: Content::new( format!( "{formated_amount} {display}"))?,
                    indent: Some(Indent::new(2)?),
                    expert: false,
                };

                Ok(vec![screen])
            }
            _ => Ok(vec![Screen {
                title: "Amount".to_string(),
                content: Content::new( format!( "{} {display}", DefaultPrimitiveRenderer::format( self.amount.clone() )) )?,
                indent: Some(Indent::new(2)?),
                expert: false,
            }]),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::signing::renderer::value_renderer::{DefaultValueRenderer, ValueRenderer};
    use anyhow::{Ok, anyhow};
    use database::{Database, PrefixDB};
    use gears::types::context::context::{Context, ContextTrait};
    use num_bigint::ToBigUint;
    use proto_messages::cosmos::{tx::v1beta1::{
        screen::{Content, Indent, Screen},
        tx_metadata::{DenomUnit, Metadata},
    }, base::v1beta1::Coin};
    use store::StoreKey;
    use strum::EnumIter;

    #[test]
    fn coin_formatting() -> anyhow::Result<()> {
        let coin = Coin {
            denom: "uatom".try_into()?,
            amount: 10000000_u64.to_biguint().ok_or( anyhow!( "Failed to parse to biguint"))?,
        };

        let expected_screens = Screen {
            title: "Amount".to_string(),
            content: Content::new("10 ATOM".to_string())?,
            indent: Some(Indent::new(2)?),
            expert: false,
        };
        let mut ctx = MockContext;

        let context: Context<'_, '_, database::RocksDB, KeyMock> =
            Context::DynamicContext(&mut ctx);

        let actual_screen = ValueRenderer::<DefaultValueRenderer, KeyMock>::format(&coin, &context);

        assert!(actual_screen.is_ok(), "Failed to retrieve screens");
        assert_eq!(vec![expected_screens], actual_screen.unwrap());

        Ok(())
    }

    // We use custom implementation instead of mock
    // 1. Mockall requires generic parameters to be 'static
    // 2. Diffuclties exporting mock on other crates
    pub struct MockContext;

    impl<T: Database, SK: StoreKey> ContextTrait<T, SK> for MockContext {
        fn height(&self) -> u64 {
            unimplemented!()
        }

        fn chain_id(&self) -> &str {
            unimplemented!()
        }

        fn push_event(&mut self, _: tendermint_informal::abci::Event) {
            unimplemented!()
        }

        fn append_events(&mut self, _: Vec<tendermint_informal::abci::Event>) {
            unimplemented!()
        }

        fn metadata_get(&self) -> Metadata {
            Metadata {
                description: String::new(),
                denom_units: vec![
                    DenomUnit {
                        denom: "ATOM".parse().expect( "Test data should be valid" ),
                        exponent: 6,
                        aliases: Vec::new(),
                    },
                    DenomUnit {
                        denom: "uatom".parse().expect( "Test data should be valid" ),
                        exponent: 0,
                        aliases: Vec::new(),
                    },
                ],
                base: "uatom".into(),
                display: "ATOM".into(),
                name: String::new(),
                symbol: String::new(),
                uri: String::new(),
                uri_hash: None,
            }
        }

        fn get_kv_store(&self, _: &SK) -> &store::KVStore<PrefixDB<T>> {
            unimplemented!()
        }

        fn get_mutable_kv_store(&mut self, _: &SK) -> &mut store::KVStore<PrefixDB<T>> {
            unimplemented!()
        }
    }

    #[derive(EnumIter, Debug, PartialEq, Eq, Hash, Clone)]
    pub enum KeyMock {
        Bank,
        Auth,
        Params,
    }

    impl StoreKey for KeyMock {
        fn name(&self) -> &'static str {
            match self {
                KeyMock::Bank => "bank",
                KeyMock::Auth => "acc",
                KeyMock::Params => "params",
            }
        }
    }
}

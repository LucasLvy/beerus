use beerus_core::{
    config::Config,
    lightclient::{
        beerus::BeerusLightClient, ethereum::MockEthereumLightClient,
        starknet::MockStarkNetLightClient,
    },
};
use cucumber::{given, then, when, World};
use ethers::types::Address;
use eyre::eyre;
use hex;
use std::{fmt, str::FromStr};
// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(World)]
#[world(init = Self::new)]
pub struct BeerusWorld {
    beerus: BeerusLightClient,
    actual: Option<String>,
}

impl fmt::Debug for BeerusWorld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BeerusWorld")
            .field("config", &self.beerus.config)
            .field("starknet light client", &self.beerus.starknet_lightclient)
            .finish()
    }
}

impl BeerusWorld {
    fn new() -> Self {
        Self {
            beerus: BeerusLightClient::new(
                Config {
                    ethereum_network: "mainnet".to_string(),
                    ethereum_consensus_rpc: "http://localhost:8545".to_string(),
                    ethereum_execution_rpc: "http://localhost:8545".to_string(),
                    starknet_rpc: "http://localhost:8545".to_string(),
                    starknet_core_contract_address: Address::from_str(
                        "0x0000000000000000000000000000000000000000",
                    )
                    .unwrap(),
                },
                Box::new(MockEthereumLightClient::new()),
                Box::new(MockStarkNetLightClient::new()),
            ),
            actual: None,
        }
    }
}

#[given("normal conditions")]
fn normal_conditions(_world: &mut BeerusWorld) {}

#[given(regex = r#"is (0x[0-9A-Fa-f]{1,63})"#)]
fn is(world: &mut BeerusWorld, root_value: String) {
    let mut ethereum_lightclient = MockEthereumLightClient::new();
    ethereum_lightclient
        .expect_call()
        .return_once(move |_call_opts, _block_tag| {
            hex::decode(root_value.trim_start_matches("0x")).map_err(|e| eyre!(e))
        });
    world.beerus.ethereum_lightclient = Box::new(ethereum_lightclient);
}

#[when("I query starknet state root")]
async fn starknet_state_root(world: &mut BeerusWorld) {
    world.actual = Some(
        world
            .beerus
            .starknet_state_root()
            .await
            .unwrap()
            .to_string(),
    );
}

#[then(regex = r#"I get (0x[0-9A-Fa-f]{1,63})"#)]
fn i_get(world: &mut BeerusWorld, result: String) {
    assert_eq!(
        world.actual.as_deref().unwrap(),
        result
            .trim_start_matches("0x")
            .parse::<u128>()
            .unwrap()
            .to_string()
    );
}

#[tokio::main]
async fn main() {
    BeerusWorld::run("tests/features/ber").await;
}

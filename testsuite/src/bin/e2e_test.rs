// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[allow(unused)]
use clap::*;
use std::collections::HashMap;
use std::{
    fs,
    path::Path,
    sync::{Arc, RwLock},
};
use sui::config::{Config, GatewayType, WalletConfig};
use sui::{keystore::KeystoreType, wallet_commands::WalletContext};
use sui_faucet::{CoinInfo, FaucetResponse};
use sui_types::{
    base_types::{encode_bytes_hex, SuiAddress},
    crypto::get_key_pair,
    gas_coin::GasCoin,
    object::{ObjectRead, Owner},
};

#[derive(Parser)]
#[clap(name = "", rename_all = "kebab-case")]
struct E2ETestOpt {
    #[clap(long)]
    gateway_url: String,
    #[clap(long)]
    faucet_url: String,
    #[clap(long, default_value_t = 9000)]
    gateway_port: u16,
    #[clap(long, default_value_t = 9100)]
    faucet_port: u16,
}

struct TestContext {
    wallet_context: WalletContext,
    address: SuiAddress,
}

// async fn test_transfer(_test_context: &TestContext, gas_coins: &Vec<GasCoin>) {
//     let (_receipent_addr, _) = get_key_pair();
// }

// async fn test_nft_creation

async fn test_get_gas(test_context: &TestContext, options: &E2ETestOpt) -> Vec<GasCoin> {
    let client = reqwest::Client::new();
    let gas_url = format!("{}:{}/gas", { &options.faucet_url }, {
        options.faucet_port
    });
    // fixme
    // let mut map = HashMap::new();
    let data = HashMap::from([("recipient", encode_bytes_hex(&test_context.address))]);
    // map.insert("FixedAmountRequest", data);
    let map = HashMap::from([("FixedAmountRequest", data)]);

    println!("Requesting for gas from {}", gas_url);
    let response = client
        .post(&gas_url)
        .json(&map)
        .send()
        .await
        .unwrap()
        .json::<FaucetResponse>()
        .await
        .unwrap();

    if let Some(error) = response.error {
        panic!("Failed to get gas tokens with error: {}", error)
    }
    let gas_coins = futures::future::join_all(
        response
            .transferred_gas_objects
            .iter()
            .map(|coin_info| verify_gas_coin(test_context, coin_info))
            .collect::<Vec<_>>(),
    )
    .await;

    gas_coins
}

/// Verify Gas Coin exists with expected value and owner
async fn verify_gas_coin(test_context: &TestContext, coin_info: &CoinInfo) -> GasCoin {
    let object_id = coin_info.id;
    let object_info = test_context
        .wallet_context
        .gateway
        .get_object_info(object_id)
        .await
        .unwrap_or_else(|err| {
            panic!(
                "Failed to get object info (id: {}) from gateway, err: {err}",
                coin_info.id
            )
        });
    match object_info {
        ObjectRead::NotExists(_) => panic!("Gateway can't find gas object {}", object_id),
        ObjectRead::Deleted(_) => panic!("Gas object {} was deleted", object_id),
        ObjectRead::Exists(_, object, _) => {
            let move_obj = object
                .data
                .try_as_move()
                .unwrap_or_else(|| panic!("Object {} is not a move object", object_id));
            let gas_coin = GasCoin::try_from(move_obj)
                .unwrap_or_else(|err| panic!("Object {} is not a gas coin, {}", object_id, err));
            if let Owner::AddressOwner(owner_address) = object.owner {
                assert_eq!(
                    owner_address, test_context.address,
                    "Gas coin {} does not belong to {}, but {}",
                    object_id, test_context.address, owner_address
                );
                gas_coin
            } else {
                panic!("Gas coin {} is not owned by AddressOwner", object_id);
            }
        }
    }
}

fn setup(options: &E2ETestOpt) -> TestContext {
    let temp_dir = tempfile::tempdir().unwrap();
    let wallet_config_path = temp_dir.path().join("wallet.conf");

    let gateway_host_port = format!("{}:{}", options.gateway_url, options.gateway_port);
    let keystore_path = temp_dir.path().join("wallet.key");
    let keystore = KeystoreType::File(keystore_path);
    let new_address = keystore.init().unwrap().add_random_key().unwrap();
    WalletConfig {
        accounts: vec![new_address],
        keystore,
        gateway: GatewayType::RPC(gateway_host_port),
        active_address: Some(new_address),
    }
    .persisted(&wallet_config_path)
    .save()
    .unwrap();

    println!(
        "Initialize wallet from config path: {:?}",
        wallet_config_path
    );

    let wallet_context = WalletContext::new(&wallet_config_path).unwrap();

    TestContext {
        wallet_context,
        address: new_address,
    }
}

#[tokio::main]
async fn main() {
    let options = E2ETestOpt::parse();
    let test_context = setup(&options);
    let _coins = test_get_gas(&test_context, &options).await;
}

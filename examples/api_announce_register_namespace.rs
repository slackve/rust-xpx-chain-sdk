#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::Client;

use xpx_chain_sdk::apis::sirius_client::SiriusClient;
use xpx_chain_sdk::models::account::Account;
use xpx_chain_sdk::models::network::PUBLIC_TEST;
use xpx_chain_sdk::models::transaction::{Deadline, RegisterNamespaceTransaction};
use xpx_chain_sdk::models::Uint64;

#[tokio::main]
async fn main() {
    let node = "http://bctestnet3.brimstone.xpxsirius.io:3000";

    let client = SiriusClient::new(node, Client::new());

    let generation_hash = client.generation_hash().await;

    // let network_type = client.network_type().await;
    let network_type = PUBLIC_TEST;

    // Deadline default 1 hour
    // let deadline = Deadline::new(1, 0, 0);
    let deadline = Deadline::default();

    let private_key = "5D3E959EB0CD69CC1DB6E9C62CB81EC52747AB56FA740CF18AACB5003429AD2E";

    let account = Account::from_private_key(private_key, network_type).unwrap();

    let register_namespace_root = RegisterNamespaceTransaction::create_root(
        deadline,
        "rustnamespace",
        Uint64::new(100),
        network_type
    );

    let namespace_root = match &register_namespace_root {
        Ok(register_namespace) => register_namespace,
        Err(err) => panic!("{}", err),
    };

    let sig_transaction_root = account.sign(namespace_root, &generation_hash);

    if let Err(err) = &sig_transaction_root {
        panic!("{}", err)
    }

    let sig_transaction = &sig_transaction_root.unwrap();

    println!("Singer: \t{}", account.public_account.public_key.to_uppercase());
    println!("Hash: \t\t{}", sig_transaction.hash.to_uppercase());

    let response_root = client.clone().transaction.announce_transaction(&sig_transaction).await;

    match response_root {
        Ok(response) => println!("{}\n", response),
        Err(err) => eprintln!("{:?}", err),
    }

    let register_namespace_sub = RegisterNamespaceTransaction::create_sub(
        deadline,
        "latam",
        namespace_root.namespace_id,
        network_type
    );

    if let Err(err) = &register_namespace_sub {
        panic!("{}", err)
    }

    let sig_transaction_sub = account.sign(&register_namespace_sub.unwrap(), &generation_hash);

    if let Err(err) = &sig_transaction_sub {
        panic!("{}", err)
    }

    let sig_transaction = &sig_transaction_sub.unwrap();

    println!("Singer: \t{}", account.public_account.public_key.to_uppercase());
    println!("Hash: \t\t{}", sig_transaction.hash.to_uppercase());

    let response_sub = client.transaction.announce_transaction(&sig_transaction).await;

    match response_sub {
        Ok(response) => println!("{}", response),
        Err(err) => eprintln!("{:?}", err),
    }
}

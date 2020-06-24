// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use xpx_chain_api::SiriusClient;
use xpx_chain_sdk::account::{
    Account, AccountPropertiesEntityTypeModification, AccountPropertyType, ADD_PROPERTY,
};
use xpx_chain_sdk::mosaic::MosaicId;
use xpx_chain_sdk::transaction::{
    AccountPropertiesEntityTypeTransaction, Deadline, EntityTypeEnum,
};

const PRIVATE_KEY: &str = "EE5D1277A862A449173C55454740BEE1A29AB837A97507021340B6EA68909097";

#[tokio::main]
async fn main() {
    let node_url = vec!["http://bctestnet1.brimstone.xpxsirius.io:3000"];

    let sirius_client = SiriusClient::new(node_url).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    let generation_hash = client.generation_hash();

    // let network_type = xpx_chain_sdk::network::PUBLIC_TEST;
    let network_type = client.network_type();

    // Deadline default 1 hour
    // let deadline = Deadline::new(1, 0, 0);
    let deadline = Deadline::default();

    let account = Account::from_private_key(PRIVATE_KEY, network_type).unwrap();

    let properties_entity_transaction = AccountPropertiesEntityTypeTransaction::new(
        deadline,
        AccountPropertyType::BlockTransaction,
        vec![AccountPropertiesEntityTypeModification::new(
            ADD_PROPERTY,
            EntityTypeEnum::AccountLink,
        )],
        network_type,
    );

    if let Err(err) = &properties_entity_transaction {
        panic!("{}", err)
    }

    let sig_transaction = account.sign(properties_entity_transaction.unwrap(), generation_hash);

    let sig_tx = match &sig_transaction {
        Ok(sig) => sig,
        Err(err) => panic!("{}", err),
    };

    println!("Singer: \t{}", account.public_key_string());
    println!("Hash: \t\t{}", sig_tx.get_hash());

    let response = client.transaction_api().announce(&sig_tx).await;

    match response {
        Ok(resp) => println!("{}", resp),
        Err(err) => eprintln!("{}", err),
    }
}
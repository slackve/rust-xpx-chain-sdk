#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::Client;

use xpx_chain_sdk::sirius_client::SiriusClient;

const NODE_URL: &str = "http://bctestnet1.brimstone.xpxsirius.io:3000";

#[tokio::main]
async fn main() {
    let sirius_client = SiriusClient::new(NODE_URL, Client::new()).await;
    let client = match sirius_client {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err),
    };

    let transaction_status = client.transaction_api().get_transaction_status(
        "C7EE3B9FB368AAA447A02E49E2F3D1CA3190300E4CDB9E5C6370C927C64682E1").await;
    match transaction_status {
        Ok(status) => println!("{}", status),
        Err(err) => eprintln!("{}", err),
    }

    let transactions_ids = vec![
        "E55037A661EE69C5F45CD3F40744F0923DD5B827F50BAF9AAF61DF649DC7B1D9",
        "5EC5C0E766B3DF81FBAD0E4FD794828002763905FEDC47208520E90FBED783B4"
    ];

    let transactions_statuses = client.transaction_api().get_transactions_statuses(
        transactions_ids.clone()).await;
    match transactions_statuses {
        Ok(statuses) => {
            statuses.iter().for_each(|status| {
                println!("{}", status)
            })
        }
        Err(err) => eprintln!("{}", err),
    }

    let transaction = client.transaction_api().get_transaction(
        "23CCC1BC5658CBD3525F0C08AB4D62E05F02AEAD076C9023F94241E8EF9887BC").await;
    match transaction {
        Ok(tx) => println!("{}", tx),
        Err(err) => eprintln!("{}", err),
    }

    let transactions = client.transaction_api().get_transactions(transactions_ids).await;
    match transactions {
        Ok(txs) => {
            txs.into_iter().for_each( |tx_info| {
                println!("{}", tx_info)
            })
        }
        Err(err) => eprintln!("{}", err),
    }
}

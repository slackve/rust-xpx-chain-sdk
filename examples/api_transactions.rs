#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::Client;

use xpx_chain_sdk::apis::sirius_client::SiriusClient;

#[tokio::main]
async fn main() {
    let node = "http://bctestnet1.brimstone.xpxsirius.io:3000";

    let client = SiriusClient::new(node, Client::new());

    let transaction_status = client.clone().transaction.get_transaction_status(
        "130171141CAE9D9ED6F62FD47CC316631986BBACD6B3D63930A9C46ED1ED764F").await;
    match transaction_status {
        Ok(status) => println!("{}", status),
        Err(err) => eprintln!("{:?}", err),
    }

    let transactions_ids = vec![
        "130171141CAE9D9ED6F62FD47CC316631986BBACD6B3D63930A9C46ED1ED764F",
        "c088f96252ab76ac73b4367de2c8c6d7dfaf33ea3e7120d867888632862eb514"
    ];

    let transactions_statuses = client.clone().transaction.get_transactions_statuses(
        transactions_ids.clone()).await;
    match transactions_statuses {
        Ok(statuses) => {
            for status in statuses {
                println!("{}", status)
            }
        }
        Err(err) => eprintln!("{:?}", err),
    }

    let transaction = client.clone().transaction.get_transaction(
        "130171141CAE9D9ED6F62FD47CC316631986BBACD6B3D63930A9C46ED1ED764F").await;
    match transaction {
        Ok(tx) => println!("{}", tx),
        Err(err) => eprintln!("{:?}", err),
    }

    let transactions = client.clone().transaction.get_transactions(transactions_ids ).await;
    match transactions {
        Ok(tx) => {
            for i in tx {
                println!("{}", i)

            }
        },
        Err(err) => eprintln!("{:?}", err),
    }
}

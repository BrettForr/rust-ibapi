use ibapi::contracts::Contract;
use ibapi::Client;

fn main() -> anyhow::Result<()> {
    let client = Client::connect("127.0.0.1:4002", 100)?;

    println!("server_version: {}", client.server_version());
    println!("connection_time: {}", client.connection_time());
    println!("managed_accounts: {}", client.managed_accounts());
    println!("next_order_id: {}", client.next_order_id());

    let mut contract = Contract::stock("TSLA");
    contract.currency = "USD".to_string();

    let results = client.contract_details(&contract)?;
    for contract in results {
        println!("contract: {contract:?}");
    }

    Ok(())
}

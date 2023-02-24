use super::*;
use crate::client::stub::ClientStub;
use crate::contracts::{contract_samples, Contract, SecurityType};

#[test]
fn place_market_order() {
    let mut client = ClientStub::new(server_versions::SIZE_RULES);

    client.response_messages = vec![
        "5|13|76792991|TSLA|STK||0|?||SMART|USD|TSLA|NMS|BUY|100|MKT|0.0|0.0|DAY||DU1236109||0||100|1376327563|0|0|0||1376327563.0/DU1236109/100||||||||||0||-1|0||||||2147483647|0|0|0||3|0|0||0|0||0|None||0||||?|0|0||0|0||||||0|0|0|2147483647|2147483647|||0||IB|0|0||0|0|PreSubmitted|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308||||||0|0|0|None|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|0||||0|1|0|0|0|||0||".to_owned(),
        "3|13|PreSubmitted|0|100|0|1376327563|0|0|100||0||".to_owned(),
        "11|-1|13|76792991|TSLA|STK||0.0|||ISLAND|USD|TSLA|NMS|00025b46.63f8f39c.01.01|20230224  12:04:56|DU1236109|ISLAND|BOT|100|196.52|1376327563|100|0|100|196.52|||||2||".to_owned(),
        "5|13|76792991|TSLA|STK||0|?||SMART|USD|TSLA|NMS|BUY|100|MKT|0.0|0.0|DAY||DU1236109||0||100|1376327563|0|0|0||1376327563.0/DU1236109/100||||||||||0||-1|0||||||2147483647|0|0|0||3|0|0||0|0||0|None||0||||?|0|0||0|0||||||0|0|0|2147483647|2147483647|||0||IB|0|0||0|0|Filled|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308||||||0|0|0|None|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|0||||0|1|0|0|0|||0||".to_owned(),
        "3|13|Filled|100|0|196.52|1376327563|0|196.52|100||0||".to_owned(),
        "5|13|76792991|TSLA|STK||0|?||SMART|USD|TSLA|NMS|BUY|100|MKT|0.0|0.0|DAY||DU1236109||0||100|1376327563|0|0|0||1376327563.0/DU1236109/100||||||||||0||-1|0||||||2147483647|0|0|0||3|0|0||0|0||0|None||0||||?|0|0||0|0||||||0|0|0|2147483647|2147483647|||0||IB|0|0||0|0|Filled|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.0|||USD||0|0|0|None|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|1.7976931348623157E308|0||||0|1|0|0|0|||0||".to_owned(),
    ];

    let contract = Contract {
        symbol: "TSLA".to_owned(),
        security_type: SecurityType::Stock,
        exchange: "SMART".to_owned(),
        currency: "USD".to_owned(),
        ..Contract::default()
    };

    let order_id = 13;
    let order = order_builder::market_order(super::Action::Buy, 100.0);

    let results = super::place_order(&mut client, order_id, &contract, &order);

    assert_eq!(
        client.request_messages[0],
        "3|13|0|TSLA|STK||0|||SMART||USD|||||BUY|100|MKT|||||||0||1|0|0|0|0|0|0|0||0||||||||0||-1|0|||0|||0|0||0||||||0|||||0|||||||||||0|||0|0|||0||0|0|0|0|||||||0|||||||||0|0|0|0|||0|"
    );

    assert!(
        results.is_ok(),
        "failed to place order: {:?}",
        results.unwrap_err()
    );

    // 5 open order
    // 3 order status
    // 11 execution data
}

#[test]
fn place_limit_order() {
    let mut client = ClientStub::new(server_versions::SIZE_RULES);

    client.response_messages = vec![];

    let order_id = 12;
    let contract = contract_samples::future_with_local_symbol();
    let order = order_builder::limit_order(super::Action::Buy, 10.0, 500.00);

    let results = super::place_order(&mut client, order_id, &contract, &order);

    assert_eq!(
        client.request_messages[0],
        "3|12|0||FUT|202303|0|||EUREX||EUR|FGBL MAR 23||||BUY|10|LMT|500||||||0||1|0|0|0|0|0|0|0||0||||||||0||-1|0|||0|||0|0||0||||||0|||||0|||||||||||0|||0|0|||0||0|0|0|0|||||||0|||||||||0|0|0|0|||0|"
    );

    assert!(
        results.is_ok(),
        "failed to place order: {:?}",
        results.unwrap_err()
    );
}

#[test]
fn place_combo_market_order() {
    let mut client = ClientStub::new(server_versions::SIZE_RULES);

    client.response_messages = vec![];

    let order_id = 12; // get next order id
    let contract = contract_samples::smart_future_combo_contract();
    let order = order_builder::combo_market_order(Action::Sell, 150.0, true);

    let results = super::place_order(&mut client, order_id, &contract, &order);

    assert_eq!(
        client.request_messages[0],
        "3|12|0|WTI|BAG||0|||SMART||USD|||||SELL|150|MKT|||||||0||1|0|0|0|0|0|0|0|2|55928698|1|BUY|IPE|0|0||0|55850663|1|SELL|IPE|0|0||0|0|1|NonGuaranteed|1||0||||||||0||-1|0|||0|||0|0||0||||||0|||||0|||||||||||0|||0|0|||0||0|0|0|0|||||||0|||||||||0|0|0|0|||0|"
    );

    assert!(
        results.is_ok(),
        "failed to place order: {:?}",
        results.unwrap_err()
    );
}

// 11:49:32:189 <- 3-12-0-AAPL-STK--0.0---SMART--USD-----BUY-100-MKT-------0--1-0-0-0-0-0-0-0--0--------0---1-0---0---0-0--0------0-----0-----------0---0-0---0--0-0-0-0--1.7976931348623157e+308-1.7976931348623157e+308-1.7976931348623157e+308-1.7976931348623157e+308-1.7976931348623157e+308-0----1.7976931348623157e+308-----0-0-0--2147483647-2147483647-0-
// 11:49:32:797 -> -- �5-12-265598-AAPL-STK--0-?--SMART-USD-AAPL-NMS-BUY-100-MKT-0.0-0.0-DAY--DU1236109--0--123-45587459-0-0-0--45587459.0/DU1236109/100----------0---1-0------2147483647-0-0-0--3-0-0--0-0--0-None--0----?-0-0--0-0------0-0-0-2147483647-2147483647---0--IB-0-0--0-0-Submitted-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308------0-0-0-None-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-0----0-1-0-0-0---0----+3-12-Submitted-0-100-0-45587459-0-0-123--0-
// 11:49:32:834 -> ---�11--1-12-265598-AAPL-STK--0.0---ISLAND-USD-AAPL-NMS-0000e0d5.64305db8.01.01-20230223  11:49:33-DU1236109-ISLAND-BOT-100-149.23-45587459-123-0-100-149.23-----2-
// 11:49:32:835 -> -- �5-12-265598-AAPL-STK--0-?--SMART-USD-AAPL-NMS-BUY-100-MKT-0.0-0.0-DAY--DU1236109--0--123-45587459-0-0-0--45587459.0/DU1236109/100----------0---1-0------2147483647-0-0-0--3-0-0--0-0--0-None--0----?-0-0--0-0------0-0-0-2147483647-2147483647---0--IB-0-0--0-0-Filled-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308------0-0-0-None-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-0----0-1-0-0-0---0----23-12-Filled-100-0-149.23-45587459-0-149.23-123--0-
// 11:49:32:836 -> -- �5-12-265598-AAPL-STK--0-?--SMART-USD-AAPL-NMS-BUY-100-MKT-0.0-0.0-DAY--DU1236109--0--123-45587459-0-0-0--45587459.0/DU1236109/100----------0---1-0------2147483647-0-0-0--3-0-0--0-0--0-None--0----?-0-0--0-0------0-0-0-2147483647-2147483647---0--IB-0-0--0-0-Filled-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.0---USD--0-0-0-None-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-1.7976931348623157E308-0----0-1-0-0-0---0----23-12-Filled-100-0-149.23-45587459-0-149.23-123--0-
// 11:49:32:837 -> ---T59-1-0000e0d5.64305db8.01.01-1.0-USD-1.7976931348623157E308-1.7976931348623157E308--

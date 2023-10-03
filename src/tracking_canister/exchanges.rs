use candid::Principal;

use crate::{custom_types::{StringStringTuple, Overview, OverviewPlus, ExchangeOverviewTotal, ExchangeCollection}, constants::ICP, utils::log};


pub fn exchangeData() -> [StringStringTuple; 26] { 

    let exchanges: [StringStringTuple; 26] = [
    StringStringTuple{st1: String::from(""), st2: String::from("Binance 1")},
    StringStringTuple{st1: String::from(""), st2: String::from("Binance Cold 1")},
    StringStringTuple{st1: String::from(""), st2: String::from("Binance Cold 2")},
    StringStringTuple{st1: String::from(""), st2: String::from("Binance Cold 3")},
    StringStringTuple{st1: String::from(""), st2: String::from("Binance Cold 4")},
    StringStringTuple{st1: String::from(""), st2: String::from("Binance Cold 5")},
    StringStringTuple{st1: String::from(""), st2: String::from("Binance 2")},
    StringStringTuple{st1: String::from(""), st2: String::from("Kucoin 1")},
    StringStringTuple{st1: String::from(""), st2: String::from("Kucoin 2")},
    StringStringTuple{st1: String::from(""), st2: String::from("Gate.io 1")},
    StringStringTuple{st1: String::from(""), st2: String::from("Coinex 1")},
    StringStringTuple{st1: String::from(""), st2: String::from("Kraken 1")},
    StringStringTuple{st1: String::from(""), st2: String::from("Kraken Cold 1")},
    StringStringTuple{st1: String::from(""), st2: String::from("Bitfinex 1")},
    StringStringTuple{st1: String::from(""), st2: String::from("Coinbase 1")},
    StringStringTuple{st1: String::from(""), st2: String::from("Coinbase 2")},
    StringStringTuple{st1: String::from(""), st2: String::from("Coinbase 3")},
    StringStringTuple{st1: String::from(""), st2: String::from("Coinbase 4")},
    StringStringTuple{st1: String::from(""), st2: String::from("Coinbase 5")},
    StringStringTuple{st1: String::from(""), st2: String::from("Coinbase 6")},
    StringStringTuple{st1: String::from(""), st2: String::from("Coinbase 7")},
    StringStringTuple{st1: String::from(""), st2: String::from("Coinbase 8")},
    StringStringTuple{st1: String::from(""), st2: String::from("Coinbase 9")},
    StringStringTuple{st1: String::from(""), st2: String::from("Coinbase 10")},
    StringStringTuple{st1: String::from(""), st2: String::from("Huobi 1")},
    StringStringTuple{st1: String::from(""), st2: String::from("Huobi 2")},  
];

    return exchanges;
}

pub async fn fetch_exchange_data() -> Vec<OverviewPlus> {
    let account_targs = exchangeData();
    let mut output: Vec<OverviewPlus> = Vec::new();

    for ac in account_targs {
        let store_id = Principal::from_text(&ICP);
        match store_id {
            Ok(pr_id) => {
                // call
                let args = ac.st1.clone();
                
                let (call_res,):(Option<Overview>,)  = ic_cdk
                ::call(pr_id, "get_overview_by_id", (args,)).await
                .map_err(|(code, str)| format!("code: {:#?} message: {}", code, str))
                .unwrap();
                
                log(format!("RES :: {:?}", call_res));

                match call_res {
                    Some(ov) => {
                        output.push(OverviewPlus { 
                            name: ac.st2, 
                            account: ac.st1, 
                            first_active: ov.first_active, 
                            last_active: ov.last_active, 
                            sent: ov.sent, 
                            received: ov.received, 
                            balance: ov.balance 
                        });
                    },
                    None => {
                        log(format!("Error fetching data for :{}", ac.st2));
                    },
                }
            },
            Err(error) => {
                log(format!("Error getting principal from string (fetch exchange overviews) Err:{}", error));
            }
        }
    }
    return output;
}

pub fn calculate_exchange_overview(vec_ovp: Vec<OverviewPlus>) -> ExchangeCollection {
    let mut binance: ExchangeOverviewTotal = ExchangeOverviewTotal { name: "".to_string(), total_balance: 0, num_transactions: 0, total_sent: 0, num_sent: 0, total_received: 0, num_received: 0 };
    let mut kucoin: ExchangeOverviewTotal = ExchangeOverviewTotal { name: "".to_string(), total_balance: 0, num_transactions: 0, total_sent: 0, num_sent: 0, total_received: 0, num_received: 0 };
    let mut gate: ExchangeOverviewTotal = ExchangeOverviewTotal { name: "".to_string(), total_balance: 0, num_transactions: 0, total_sent: 0, num_sent: 0, total_received: 0, num_received: 0 };
    let mut coinex: ExchangeOverviewTotal = ExchangeOverviewTotal { name: "".to_string(), total_balance: 0, num_transactions: 0, total_sent: 0, num_sent: 0, total_received: 0, num_received: 0 };
    let mut kraken: ExchangeOverviewTotal = ExchangeOverviewTotal { name: "".to_string(), total_balance: 0, num_transactions: 0, total_sent: 0, num_sent: 0, total_received: 0, num_received: 0 };
    let mut bitfinex: ExchangeOverviewTotal = ExchangeOverviewTotal { name: "".to_string(), total_balance: 0, num_transactions: 0, total_sent: 0, num_sent: 0, total_received: 0, num_received: 0 };
    let mut coinbase: ExchangeOverviewTotal = ExchangeOverviewTotal { name: "".to_string(), total_balance: 0, num_transactions: 0, total_sent: 0, num_sent: 0, total_received: 0, num_received: 0 };
    let mut huobi: ExchangeOverviewTotal = ExchangeOverviewTotal { name: "".to_string(), total_balance: 0, num_transactions: 0, total_sent: 0, num_sent: 0, total_received: 0, num_received: 0 };

    let acs = exchangeData();
    
    // verify all accounts loaded 
    let mut is_ok = true;
    for i in 0..acs.len() {
        if vec_ovp[i].account != acs[i].st1 {
            is_ok = false;
        }
    }

    if is_ok == true {
    // data is complete
    let mut total_balance: u64; 
    let mut num_transactions: u64;
    let mut total_sent: u64;
    let mut num_sent: u64;
    let mut total_received: u64;
    let mut num_received: u64;

    // BINANCE
    total_balance = (vec_ovp[0].balance + vec_ovp[1].balance + vec_ovp[2].balance 
    + vec_ovp[3].balance + vec_ovp[4].balance + vec_ovp[5].balance + vec_ovp[6].balance);

    num_sent = (vec_ovp[0].sent.0 + vec_ovp[1].sent.0 + vec_ovp[2].sent.0 
        + vec_ovp[3].sent.0 + vec_ovp[4].sent.0 + vec_ovp[5].sent.0 + vec_ovp[6].sent.0) as u64;

    total_sent = (vec_ovp[0].sent.1 + vec_ovp[1].sent.1 + vec_ovp[2].sent.1 
        + vec_ovp[3].sent.1 + vec_ovp[4].sent.1 + vec_ovp[5].sent.1 + vec_ovp[6].sent.1);

    num_received = (vec_ovp[0].received.0 + vec_ovp[1].received.0 + vec_ovp[2].received.0 
        + vec_ovp[3].received.0 + vec_ovp[4].received.0 + vec_ovp[5].received.0 + vec_ovp[6].received.0) as u64;
    
    total_received = (vec_ovp[0].received.1 + vec_ovp[1].received.1 + vec_ovp[2].received.1 
        + vec_ovp[3].received.1 + vec_ovp[4].received.1 + vec_ovp[5].received.1 + vec_ovp[6].received.1);

    num_transactions = (num_sent+num_received);
    
    binance = ExchangeOverviewTotal{
        name: "Binance".to_string(),
        total_balance,
        num_transactions,
        total_sent,
        num_sent,
        total_received,
        num_received,
    };
            
    // Kucoin  
    total_balance = (vec_ovp[7].balance + vec_ovp[8].balance);

    num_sent = (vec_ovp[7].sent.0 + vec_ovp[8].sent.0 ) as u64;

    total_sent = (vec_ovp[7].sent.1 + vec_ovp[8].sent.1 );

    num_received = (vec_ovp[7].received.0 + vec_ovp[8].received.0) as u64;
    
    total_received = (vec_ovp[7].received.1 + vec_ovp[8].received.1);

    num_transactions = (num_sent+num_received);
    
    kucoin = ExchangeOverviewTotal{
        name: "Kucoin".to_string(),
        total_balance,
        num_transactions,
        total_sent,
        num_sent,
        total_received,
        num_received,
    };
            
    // Gate.io
    total_balance = vec_ovp[9].balance;

    num_sent =vec_ovp[9].sent.0 as u64;

    total_sent = vec_ovp[9].sent.1;

    num_received = vec_ovp[9].received.0 as u64;
    
    total_received = vec_ovp[9].received.1;

    num_transactions = (num_sent+num_received);
    
    gate = ExchangeOverviewTotal{
        name: "Gate.io".to_string(),
        total_balance,
        num_transactions,
        total_sent,
        num_sent,
        total_received,
        num_received,
    };
        
    
    // Coinex
    total_balance = vec_ovp[10].balance;

    num_sent =vec_ovp[10].sent.0 as u64;

    total_sent = vec_ovp[10].sent.1;

    num_received = vec_ovp[10].received.0 as u64;
    
    total_received = vec_ovp[10].received.1;

    num_transactions = (num_sent+num_received);
    
    coinex = ExchangeOverviewTotal{
        name: "Coinex".to_string(),
        total_balance,
        num_transactions,
        total_sent,
        num_sent,
        total_received,
        num_received,
    };
        
    // Kraken
    total_balance = (vec_ovp[11].balance + vec_ovp[12].balance);

    num_sent = (vec_ovp[11].sent.0 + vec_ovp[12].sent.0 ) as u64;

    total_sent = (vec_ovp[11].sent.1 + vec_ovp[12].sent.1 );

    num_received = (vec_ovp[11].received.0 + vec_ovp[12].received.0) as u64;
    
    total_received = (vec_ovp[11].received.1 + vec_ovp[12].received.1);

    num_transactions = (num_sent+num_received);
    
    kraken = ExchangeOverviewTotal{
        name: "Kraken".to_string(),
        total_balance,
        num_transactions,
        total_sent,
        num_sent,
        total_received,
        num_received,
    };
        
    
    // Bitfinex
    total_balance = vec_ovp[13].balance;

    num_sent =vec_ovp[13].sent.0 as u64;

    total_sent = vec_ovp[13].sent.1;

    num_received = vec_ovp[13].received.0 as u64;
    
    total_received = vec_ovp[13].received.1;

    num_transactions = (num_sent+num_received);
    
    bitfinex = ExchangeOverviewTotal{
        name: "Bitfinex".to_string(),
        total_balance,
        num_transactions,
        total_sent,
        num_sent,
        total_received,
        num_received,
    };
        
// Coinbase
    total_balance = (vec_ovp[14].balance + vec_ovp[15].balance + vec_ovp[16].balance 
    + vec_ovp[17].balance + vec_ovp[18].balance + vec_ovp[19].balance + vec_ovp[20].balance + 
    vec_ovp[21].balance + vec_ovp[22].balance + vec_ovp[23].balance 
);

    num_sent = (
    vec_ovp[14].sent.0 + vec_ovp[15].sent.0 + vec_ovp[16].sent.0 
    + vec_ovp[17].sent.0 + vec_ovp[18].sent.0 + vec_ovp[19].sent.0 + vec_ovp[20].sent.0 + 
    vec_ovp[21].sent.0 + vec_ovp[22].sent.0 + vec_ovp[23].sent.0 
    ) as u64;

    total_sent = (vec_ovp[14].sent.1 + vec_ovp[15].sent.1 + vec_ovp[16].sent.1 
    + vec_ovp[17].sent.1 + vec_ovp[18].sent.1 + vec_ovp[19].sent.1 + vec_ovp[20].sent.1 + 
    vec_ovp[21].sent.1 + vec_ovp[22].sent.1 + vec_ovp[23].sent.1);

num_received = (vec_ovp[14].received.0 + vec_ovp[15].received.0 + vec_ovp[16].received.0 
    + vec_ovp[17].received.0 + vec_ovp[18].received.0 + vec_ovp[19].received.0 + vec_ovp[20].received.0 + 
    vec_ovp[21].received.0 + vec_ovp[22].received.0 + vec_ovp[23].received.0) as u64;
    
total_received = (vec_ovp[14].received.1 + vec_ovp[15].received.1 + vec_ovp[16].received.1 
    + vec_ovp[17].received.1 + vec_ovp[18].received.1 + vec_ovp[19].received.1 + vec_ovp[20].received.1 + 
    vec_ovp[21].received.1 + vec_ovp[22].received.1 + vec_ovp[23].received.1);

    num_transactions = (num_sent+num_received);
    
    coinbase = ExchangeOverviewTotal{
        name: "Coinbase".to_string(),
        total_balance,
        num_transactions,
        total_sent,
        num_sent,
        total_received,
        num_received,
    };
         
    // Huobi
    total_balance = (vec_ovp[24].balance + vec_ovp[25].balance);

    num_sent = (vec_ovp[24].sent.0 + vec_ovp[25].sent.0 ) as u64;

    total_sent = (vec_ovp[24].sent.1 + vec_ovp[25].sent.1 );

    num_received = (vec_ovp[24].received.0 + vec_ovp[25].received.0) as u64;
    
    total_received = (vec_ovp[24].received.1 + vec_ovp[25].received.1);

    num_transactions = (num_sent+num_received);
    
    huobi = ExchangeOverviewTotal{
        name: "Huobi".to_string(),
        total_balance,
        num_transactions,
        total_sent,
        num_sent,
        total_received,
        num_received,
    };
    
    } else {
    // data is incomplete
        log("Error - Huobi data is incomplete! (calculate_exchange_overview)");
    }

  let ret = ExchangeCollection{ binance, kucoin, gate, 
                            coinex, kraken, bitfinex, coinbase, huobi };
  return ret;                       
}
use chain::transaction::Transaction;
use chain::Blockchain;

pub mod chain;

fn main() {
    let todays_transactions = vec![
        Transaction::new("Lucas".to_string(), "Jack".to_string(), 1.2),
        Transaction::new("Jack".to_string(), "Ruby".to_string(), 33.3),
        Transaction::new("Lucas".to_string(), "Ruby".to_string(), 1.0),
        Transaction::new("Nick".to_string(), "Rufus".to_string(), 10.00),
    ];

    let mut truthcoin = Blockchain::new(1, 2.0);

    truthcoin.stake("Jocie".to_string(), 32.0);
    truthcoin.stake("Linda".to_string(), 40.2);
    truthcoin.stake("Mark".to_string(), 1000.0);
    truthcoin.unstake("Mark".to_string(), 10.0);

    todays_transactions
        .iter()
        .for_each(|tx| truthcoin.add_transaction(tx.clone()));

    for _ in 1..3 {
        truthcoin.mine_block();
    }

    let valid = truthcoin.is_chain_valid();
    println!("Chain is valid check: [{}]", valid);
}

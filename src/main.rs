pub mod chain;
pub mod wallet;

use std::fmt;

use crate::chain::Chain;
// use wallet::Wallet;

pub struct Transaction {
    amount: i32,
    payer: String,
    payee: String,
}

impl Transaction {
    fn new(amount: i32, payer: String, payee: String) -> Self {
        Self {
            amount,
            payee,
            payer,
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Amount: {}, Payee: {}, Payer: {}",
            self.amount, self.payee, self.payer
        )
    }
}

pub fn main() {
    let t = Transaction {
        amount: 100,
        payer: String::from("Lucas"),
        payee: String::from("Oliver"),
    };
    println!("{}", t);

    let chain = Chain::new();
    let nonce = 2192; // Example nonce value
    chain.mine(nonce);
}

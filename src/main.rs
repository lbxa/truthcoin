pub mod wallet;

use chrono::{DateTime, Utc};
use std::fmt;
use wallet::Wallet;

// class Block {
//     public nonce = Math.round(Math.random() * 999999999);

//     constructor(
//         public prevHash: string,
//         public transaction: Transaction,
//         public ts = Date.now()
//     ) {}

//     get hash() {
//         const str = JSON.stringify(this);
//         const hash = crypto.createHash("SHA256");
//         hash.update(str).end();
//         return hash.digest("hex");
//     }
// }

pub struct Transaction {
    amount: i32,
    payer: String,
    payee: String,
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

struct Block {
    prev_hash: String,
    transaction: Transaction,
    ts: DateTime<Utc>,
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Block {
    fn new(prev_hash: String, transaction: Transaction) -> Block {
        Block {
            prev_hash,
            transaction,
            ts: Utc::now(),
        }
    }
    fn get_hash() -> String {
        return "Hello".to_string();
    }
}

pub fn main() {
    let t = Transaction {
        amount: 100,
        payer: String::from("Lucas"),
        payee: String::from("Oliver"),
    };
    println!("{}", t);
}

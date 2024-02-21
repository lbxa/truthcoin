use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};

use crate::Transaction;

pub struct Block {
    pub prev_hash: String,
    pub transaction: Transaction,
    pub ts: DateTime<Utc>,
}

impl Block {
    pub fn new(prev_hash: String, transaction: Transaction) -> Block {
        Block {
            prev_hash,
            transaction,
            ts: Utc::now(),
        }
    }

    pub fn hash(&self) -> String {
        let str = serde_json::to_string(&self.ts.to_string()).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(str.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result) // Convert the hash result to a hexadecimal string
    }
}

pub struct Chain {
    chain: Vec<Block>,
}

impl Chain {
    pub fn new() -> Self {
        let t = Transaction::new(100, "genesis".to_string(), "satoshi".to_string());
        let block = Block::new("".to_string(), t);
        Self { chain: vec![block] }
    }

    pub fn last_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    pub fn add_block(
        &mut self,
        transaction: Transaction,
        senderPublicKey: String,
        signature: String,
    ) -> () {
        if let Some(last) = self.last_block() {
            let new_block = Block::new(last.hash(), transaction);
            self.chain.push(new_block);
        }
    }

    pub fn mine(&self, nonce: u64) -> u64 {
        let mut solution = 1;
        println!("⛏️  mining...");

        loop {
            let mut hasher = Sha256::new();
            hasher.update((nonce + solution).to_string());
            let attempt = hasher.finalize();

            let attempt_str = format!("{:x}", attempt);
            if attempt_str.starts_with("0000") {
                println!("Solved: {}", solution);
                return solution;
            }

            solution += 1;
        }
    }
}

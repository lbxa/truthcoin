use chrono::Utc;
use sha2::{Digest, Sha256};
use transaction::Transaction;

use super::transaction;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub transactions: Vec<Transaction>,
    pub timestamp: i64,
    pub validator: String,
    pub hash: String,
}

impl Block {
    pub fn new(
        index: u64,
        previous_hash: String,
        validator: String,
        transactions: Vec<Transaction>,
    ) -> Self {
        let mut block = Block {
            index,
            previous_hash,
            transactions,
            validator,
            hash: String::new(),
            timestamp: Utc::now().timestamp(),
        };
        block.hash = block.calculate_hash();
        return block;
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let input = format!(
            "{}{}{:?}{}{}",
            self.index, self.timestamp, self.transactions, self.previous_hash, self.validator
        );
        hasher.update(input);
        return format!("{:x}", hasher.finalize());
    }
}

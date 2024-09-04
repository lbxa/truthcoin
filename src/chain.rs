use block::Block;
use rand::Rng;
use std::collections::{HashMap, VecDeque};
use transaction::Transaction;

pub mod block;
pub mod transaction;

#[derive(Debug, Clone)]
pub struct Blockchain {
    chain: Vec<Block>,
    transactions_per_block: usize,
    mempool: VecDeque<Transaction>,
    validators: HashMap<String, f64>,
    total_staked: f64,
    reward_amount: f64,
}

impl Blockchain {
    pub fn new(transactions_per_block: usize, reward_amount: f64) -> Self {
        let genesis_block = Block::new(
            0,
            String::from("0"),
            String::from("Genesis"),
            vec![Transaction::new(
                String::from("Genesis"),
                String::from("Satoshi"),
                100.0,
            )],
        );
        return Blockchain {
            chain: vec![genesis_block],
            transactions_per_block,
            total_staked: 0.0,
            reward_amount,
            validators: HashMap::new(),
            mempool: VecDeque::new(),
        };
    }

    pub fn stake(&mut self, validator: String, amount: f64) {
        let current_stake = self.validators.entry(validator.clone()).or_insert(0.0);
        *current_stake += amount;
        self.total_staked += amount;
        println!(
            "{} staked {} coins. Total stake: {}",
            validator, amount, current_stake
        );
    }

    pub fn unstake(&mut self, validator: String, amount: f64) {
        if let Some(current_stake) = self.validators.get_mut(&validator) {
            if *current_stake >= amount {
                *current_stake -= amount;
                self.total_staked -= amount;
                println!(
                    "{} unstaked {} coins. Remaining stake: {}",
                    validator, amount, current_stake
                );
            } else {
                println!("Error: Not enough staked coins to unstake");
            }
        } else {
            println!("Error: Validator not found");
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.mempool.push_back(transaction);
    }

    pub fn mine_block(&mut self) {
        let mut transactions = Vec::new();
        for _ in 0..self.transactions_per_block {
            if let Some(tx) = self.mempool.pop_front() {
                transactions.push(tx);
            } else {
                break;
            }
        }

        if !transactions.is_empty() {
            if let Some(validator) = self.select_validator() {
                let previous_block = self.chain.last().unwrap();
                let new_block = Block::new(
                    previous_block.index + 1,
                    previous_block.hash.clone(),
                    validator.clone(),
                    transactions,
                );
                self.chain.push(new_block);

                // reward the validator
                if let Some(stake) = self.validators.get_mut(&validator) {
                    *stake += self.reward_amount;
                    self.total_staked += self.reward_amount;
                    println!(
                        "{} was selected as validator and received {} coins as reward",
                        validator, self.reward_amount
                    );
                }
            } else {
                println!("Error: No validators available to mine the block");
            }
        }
    }

    pub fn select_validator(&self) -> Option<String> {
        let total_stake = self.validators.values().sum();
        if total_stake == 0.0 {
            return None;
        }

        let mut rng = rand::thread_rng();
        let mut random_point = rng.gen_range(0.0..total_stake);

        for (validator, &stake) in self.validators.iter() {
            // make a weighted selection
            if random_point < stake {
                return Some(validator.clone());
            }
            random_point -= stake;
        }
        return None;
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != current_block.calculate_hash()
                || current_block.previous_hash != previous_block.hash
            {
                return false;
            }
        }

        return true;
    }
}

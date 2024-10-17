use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transaction {
    sender: String,
    recipient: String,
    amount: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Block {
    index: u32,
    transactions: Vec<Transaction>,
    timestamp: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u32, transactions: Vec<Transaction>, previous_hash: &str) -> Self {
        let timestamp = Utc::now().to_string();
        let hash = Block::calculate_hash(index, &transactions, &timestamp, previous_hash);
        
        Block {
            index,
            transactions,
            timestamp,
            previous_hash: previous_hash.to_string(),
            hash,
        }
    }

    fn calculate_hash(index: u32, transactions: &[Transaction], timestamp: &str, previous_hash: &str) -> String {
        let data = format!("{:?}{:?}{}{}", index, transactions, timestamp, previous_hash);
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
    transaction_pool: Vec<Transaction>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new(0, vec![], "0");
        Blockchain {
            chain: vec![genesis_block],
            transaction_pool: vec![],
        }
    }

    fn add_transaction(&mut self, transaction: Transaction) {
        self.transaction_pool.push(transaction);
    }

    fn mine_block(&mut self) {
        let block = Block::new(
            self.chain.len() as u32,
            self.transaction_pool.clone(),
            &self.chain.last().unwrap().hash,
        );
        self.chain.push(block);
        self.transaction_pool.clear();
    }

    fn display_chain(&self) {
        for block in &self.chain {
            println!("Block {}:", block.index);
            println!("  Hash: {}", block.hash);
            println!("  Previous Hash: {}", block.previous_hash);
            println!("  Timestamp: {}", block.timestamp);
            println!("  Transactions: {:?}", block.transactions);
        }
    }
}

fn main() {
    let mut blockchain = Blockchain::new();

    // Adding transactions
    blockchain.add_transaction(Transaction { sender: String::from("Alice"), recipient: String::from("Bob"), amount: 50.0 });
    blockchain.add_transaction(Transaction { sender: String::from("Charlie"), recipient: String::from("Dave"), amount: 75.0 });

    // Mining a block
    blockchain.mine_block();

    // Displaying the blockchain
    println!("Blockchain:");
    blockchain.display_chain();
}

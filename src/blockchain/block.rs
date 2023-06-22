use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use chrono::prelude::*;

// transaction object for data in a block
#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: f32) -> Self {
        Transaction {
            sender: sender,
            receiver: receiver,
            amount: amount,
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    index: u32,
    timestamp: DateTime<Utc>,
    transaction: Transaction,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

impl Block {
    pub fn new(index: u32, timestamp: DateTime<Utc>, transaction: Transaction, previous_hash: String) -> Self {
        let mut block = Block {
            index: index,
            timestamp: timestamp,
            transaction: transaction,
            previous_hash: previous_hash,
            hash: "".to_string(),
            nonce: 0,
        };

        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        // converting a transaction to a string
        let transaction_string = format!("{:?}", self.transaction);
        let data = format!("{}{}{}{}{}", self.index, self.timestamp, transaction_string, self.previous_hash, self.nonce);
        hasher.update(data);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub fn set_hash(&mut self, hash: String) {
        self.hash = hash;
    }

    pub fn set_previous_hash(&mut self, previous_hash: String) {
        self.previous_hash = previous_hash;
    }

    fn update_nonce(&mut self, change_to_nonce: u64) {
        self.nonce += change_to_nonce;
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn get_previous_hash(&self) -> String {
        self.previous_hash.clone()
    }

    pub fn mine_block(&mut self, difficulty: u32) -> String {
        // difficulty decides the number of leading zeros in the hash
        let mut hash = self.calculate_hash();
        while !hash.starts_with(&"0".repeat(difficulty as usize)) {
            self.update_nonce(1);
            hash = self.calculate_hash();
        }
        self.set_hash(hash.clone());
        hash
    }
}
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
}

impl Block {
    pub fn new(index: u32, timestamp: DateTime<Utc>, transaction: Transaction, previous_hash: String) -> Self {
        let mut block = Block {
            index: index,
            timestamp: timestamp,
            transaction: transaction,
            previous_hash: previous_hash,
            hash: "".to_string(),
        };

        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        // converting a transaction to a string
        let transaction_string = format!("{:?}", self.transaction);
        let data = format!("{}{}{}{}", self.index, self.timestamp, transaction_string, self.previous_hash);
        hasher.update(data);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub fn update_previous_hash_field(&mut self, previous_hash: String) {
        self.previous_hash = previous_hash;
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }
}
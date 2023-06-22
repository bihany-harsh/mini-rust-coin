mod blockchain {
    pub mod block;
    pub mod blockchain;
}

use blockchain::block::{Block, Transaction};
use blockchain::blockchain::BlockChain;
use chrono::prelude::*;

fn main() {
    let mut blockchain = BlockChain::new();
    let transaction = Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0);
    let block = Block::new(1, Utc::now(), transaction, blockchain.get_latest_block().get_hash());
    blockchain.add_block(block);

    let transaction = Transaction::new("Bob".to_string(), "Charlie".to_string(), 5.0);
    let block = Block::new(2, Utc::now(), transaction, blockchain.get_latest_block().get_hash());
    blockchain.add_block(block);

    println!("{:#?}", blockchain);
}

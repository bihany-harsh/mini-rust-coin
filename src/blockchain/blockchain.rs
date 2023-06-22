use chrono::prelude::*;
use super::block::{Block, Transaction};

#[derive(Debug)]
pub struct BlockChain {
    chain: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        let mut blockchain = BlockChain {
            chain: Vec::new(),
        };
        blockchain.chain.push(BlockChain::create_genesis_block());
        blockchain
    }
    
    // to create a genesis block
    fn create_genesis_block() -> Block {
        let transaction = Transaction::new("Genesis".to_string(), "Genesis".to_string(), 0.0);
        Block::new(0, Utc::now(), transaction, "0".to_string())
    }

    // to get the latest block
    pub fn get_latest_block(&self) -> &Block {
        // getting a block with error handling
        match self.chain.last() {
            Some(block) => block,
            None => panic!("There is no block in the chain"),
        }
    }

    // to add a new block
    pub fn add_block(&mut self, mut new_block: Block) {
        // setting the hash of the previous block as the previous_hash of the new block
        let prev_hash = self.get_latest_block().get_hash();
        new_block.update_previous_hash_field(prev_hash);
        new_block.calculate_hash();
        self.chain.push(new_block);
    }
}
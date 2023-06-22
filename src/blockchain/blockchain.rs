use chrono::prelude::*;
use super::block::{Block, Transaction};

#[derive(Debug)]
pub struct BlockChain {
    chain: Vec<Block>,
    difficulty: u32,
}

impl BlockChain {
    pub fn new() -> Self {
        let mut blockchain = BlockChain {
            chain: Vec::new(),
            difficulty: 4,
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
        new_block.set_previous_hash(self.get_latest_block().get_hash());
        // instead of setting the hash, we have to mine the block, which sets the hash
        new_block.mine_block(self.difficulty);
        new_block.set_hash(new_block.calculate_hash());
        // adding the block to the chain
        self.chain.push(new_block);
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            // checking if the current block's hash is valid
            if current_block.get_hash() != current_block.calculate_hash() {
                return false;
            }

            // checking if the previous block's hash is valid
            if current_block.get_previous_hash() != previous_block.get_hash() {
                return false;
            }
        }
        true
    }
}
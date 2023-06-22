use chrono::Utc;
// enabling std for serialization and deserialization
use super::block::{Block, Transaction};

#[derive(Debug)]
#[allow(dead_code)]
pub struct BlockChain {
    chain: Vec<Block>,
    difficulty: u32,
    pending_transactions: Vec<Transaction>,
    mining_reward: f32,
}

#[allow(dead_code)]
impl BlockChain {
    pub fn new() -> Self {
        let mut blockchain = BlockChain {
            chain: Vec::new(),
            difficulty: 5,
            pending_transactions: Vec::new(),
            mining_reward: 100.0,
        };
        blockchain.chain.push(BlockChain::create_genesis_block());
        blockchain
    }
    
    // to create a genesis block
    fn create_genesis_block() -> Block {
        let transactions = vec![Transaction::new("Genesis".to_string(), "Genesis".to_string(), 0.0)];
        Block::new(Utc::now(), transactions, "0".to_string())
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
    // pub fn add_block(&mut self, mut new_block: Block) {
    //     new_block.set_previous_hash(self.get_latest_block().get_hash());
    //     // instead of setting the hash, we have to mine the block, which sets the hash
    //     new_block.mine_block(self.difficulty);
    //     new_block.set_hash(new_block.calculate_hash());
    //     // adding the block to the chain
    //     self.chain.push(new_block);
    // }

    pub fn mine_pending_transaction(&mut self, mining_reward_address: String) {
        let mut block = Block::new(Utc::now(), self.pending_transactions.clone(), self.get_latest_block().get_hash());
        println!("Mining a block... (by: {})", mining_reward_address);
        block.mine_block(self.difficulty);
        println!("Block successfully mined!");
        self.chain.push(block);

        // resetting the pending transactions and adding the mining reward
        self.pending_transactions = Vec::new();
        self.pending_transactions.push(Transaction::new("mini-rust-coin_system".to_string(), mining_reward_address, self.mining_reward));
    }

    pub fn create_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    pub fn get_address_balance(&self, address: String) -> f32 {
        let mut balance = 0.0;
        for block in &self.chain {
            for transaction in &block.get_transactions() {
                if transaction.get_sender() == address {
                    balance -= transaction.get_amount();
                }
                if transaction.get_receiver() == address {
                    balance += transaction.get_amount();
                }
            }
        }
        balance
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
mod blockchain {
    pub mod block;
    pub mod blockchain;
}

use blockchain::block::{Transaction};
use blockchain::blockchain::BlockChain;

fn main() {
    let mut blockchain = BlockChain::new();
    blockchain.create_transaction(Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0));
    blockchain.create_transaction(Transaction::new("Bob".to_string(), "Charlie".to_string(), 15.0));
    blockchain.mine_pending_transaction("Miner1".to_string());

    println!("Miner1's balance: {}", blockchain.get_address_balance("Miner1".to_string()));
    println!("{:#?}", blockchain);

    blockchain.create_transaction(Transaction::new("Charlie".to_string(), "Alice".to_string(), 20.0));
    blockchain.create_transaction(Transaction::new("Alice".to_string(), "Darwin".to_string(), 10.0));
    blockchain.mine_pending_transaction("Miner1".to_string());

    println!("Miner1's balance: {}", blockchain.get_address_balance("Miner1".to_string()));
    println!("{:#?}", blockchain);

    blockchain.mine_pending_transaction("Miner2".to_string());

    println!("{:#?}", blockchain);

    println!("Is blockchain valid? {}", blockchain.is_chain_valid());       
}

use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};

#[derive(Debug)]
struct Block {
    index: u128,
    data: String,
    hash: String,
    previous: String,
    nonce: u64,
    timestamp: DateTime<Utc>
}

fn main() {
    let mut blockchain: Vec<Block> = Vec::new();

    let genesis_block: Block = create_genesis_block();
    println!("Genesis {:#?}", genesis_block);
    blockchain.push(genesis_block);

    blockchain.push(create_block(&blockchain, "Transaction Info".to_string()));
    blockchain.push(create_block(&blockchain, "Hello World".to_string()));
    blockchain.push(create_block(&blockchain, "Cool data".to_string()));
    blockchain.push(create_block(&blockchain, "Rust the best".to_string()));
    blockchain.push(create_block(&blockchain, "blockchain-simulator".to_string()));

    validate_blockchain(&blockchain);
}

fn create_block(blockchain: &[Block], data: String) -> Block {    
    let mut block = Block {
        index: blockchain.len() as u128,
        data,
        hash: String::new(),
        previous: blockchain[blockchain.len() - 1].hash.clone(),
        nonce: 0,
        timestamp: Utc::now()
    };

    let nonce_hash = find_nonce(&block);
    block.nonce = nonce_hash.0;
    block.hash = nonce_hash.1;

    // println!("{:#?}", block);

    block
}

fn create_genesis_block() -> Block {
    let mut genesis_block = Block {
        index: 0,
        data: String::from("Genesis Block"),
        hash: String::new(),
        previous: "0".repeat(64),
        nonce: 0,
        timestamp: Utc::now()
    };

    let nonce_hash = find_nonce(&genesis_block);
    genesis_block.nonce = nonce_hash.0;
    genesis_block.hash = nonce_hash.1;

    genesis_block
}

fn hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn find_nonce(block: &Block) -> (u64, String) {
    let mut nonce = 0;
    let mut hash_string; 
    
    loop {
        hash_string = hash(&format!("{}{}{}{}{}", nonce, block.index, block.data, block.timestamp, block.previous));
        
        if hash_string.starts_with("000") {
            break;
        }
        
        nonce += 1;
        
        if nonce % 1_000_000 == 0 {
            println!("Nonce: {} / Hash: {}", nonce, hash_string);
        }
    }

    (nonce, hash_string)
}

fn validate_blockchain(blockchain: &[Block]) {
    let mut previous_block_hash: String = String::new();

    for block in blockchain {
        // println!("{:#?}", block);
        if block.index == 0 {
            previous_block_hash = block.hash.clone();
            continue;
        }

        if block.previous == previous_block_hash {
            println!("Block {} is valid", block.index);
            previous_block_hash = block.hash.clone();
        } else {
            println!("Block {} is invalid", block.index);
            break;
        }
    }
}
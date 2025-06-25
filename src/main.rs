use axum::routing::post;
use axum::Router;
use chrono::{DateTime, Utc};
use rocket::tokio;
use sha2::{Sha256, Digest};
use axum::{
    routing::get,
    Json
};
use hyper::server;
use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, OpenApi};
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa_swagger_ui::SwaggerUi;
use utoipa::Modify;
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Block {
    index: u128,
    data: String,
    hash: String,
    previous: String,
    nonce: u64,
    timestamp: DateTime<Utc>
}

#[derive(Deserialize, ToSchema)]
pub struct CreateBlockRequest {
    // Your fields here, for example:
    data: String,
}

#[derive(OpenApi)]
#[openapi(
    paths(create_block, get_blockchain),
    components(schemas(CreateBlockRequest))
)]
struct ApiDoc;

static BLOCKCHAIN: Lazy<Mutex<Vec<Block>>> = Lazy::new(|| Mutex::new(Vec::new()));

#[utoipa::path(
    post,
    path = "/api/blockchain",
    request_body=CreateBlockRequest
)]
async fn create_block(Json(payload): Json<CreateBlockRequest>) -> Json<Block> {
    let mut blockchain = BLOCKCHAIN.lock().unwrap();

    let previous_hash = if blockchain.is_empty() {
        "0".repeat(64)
    } else {
        blockchain[blockchain.len() - 1].hash.clone()
    };

    let new_block = initialize_block(
        blockchain.len() as u128,
        payload.data,
        previous_hash
    );

    blockchain.push(new_block.clone());

    Json(new_block)
}

#[utoipa::path(
    get,
    path="/api/blockchain"
)]
async fn get_blockchain() -> Json<Vec<Block>> {
    let blockchain = BLOCKCHAIN.lock().unwrap();
    Json(blockchain.clone())
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/blockchain", post(create_block))
        .route("/api/blockchain", get(get_blockchain))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// fn create_block(blockchain: &[Block], data: String) -> Block {    
//     let previous_hash = if blockchain.is_empty() {
//         "0".repeat(64)
//     } else {
//         blockchain[blockchain.len() - 1].hash.clone()
//     };

//     initialize_block(
//         blockchain.len() as u128,
//         data,
//         previous_hash
//     )
// }

fn initialize_block(index: u128, data: String, previous: String) -> Block {
    let mut block = Block {
        index,
        data,
        hash: String::new(),
        previous,
        nonce: 0,
        timestamp: Utc::now()
    };

    let nonce_hash = find_nonce(&block);
    block.nonce = nonce_hash.0;
    block.hash = nonce_hash.1;

    block
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

fn is_blockchain_valid(blockchain: &[Block]) -> bool {
    let mut previous_block_hash: String = String::new();

    for block in blockchain {
        // println!("{:#?}", block);
        if block.index == 0 {
            previous_block_hash = block.hash.clone();
            continue;
        }

        if block.previous != previous_block_hash {
            println!("Block {} is invalid", block.index);
            return false;
        }

        println!("Block {} is valid", block.index);
        previous_block_hash = block.hash.clone();
    }

    true
}
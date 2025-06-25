use axum::{Json, Router, routing::{get, post}};
use super::service;
use crate::{blockchain::model::Block, dtos::requests::create_block::CreateBlockRequest};

pub fn blockchain_routes() -> Router {
    Router::new()
        .route("/api/blockchain", post(create_block))
        .route("/api/blockchain", get(get_blockchain))
}

#[utoipa::path(
    post,
    path = "/api/blockchain",
    request_body = CreateBlockRequest,
    responses(
        (status = 200, description = "Created new block", body = Block)
    )
)]
async fn create_block(Json(payload): Json<CreateBlockRequest>) -> Json<Block> {
    Json(service::add_block(payload.data))
}

#[utoipa::path(
    get,
    path = "/api/blockchain",
    responses(
        (status = 200, description = "List of all blocks", body = [Block])
    )
)]
async fn get_blockchain() -> Json<Vec<Block>> {
    Json(service::get_blockchain())
}
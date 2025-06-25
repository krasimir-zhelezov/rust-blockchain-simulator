mod api_doc;
mod blockchain;
mod dtos;

use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(blockchain::routes::blockchain_routes())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", api_doc::ApiDoc::openapi()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
use utoipa::OpenApi;
use crate::dtos::requests::create_block::CreateBlockRequest;
use crate::blockchain::model::Block;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::blockchain::routes::create_block,
        crate::blockchain::routes::get_blockchain
    ),
    components(
        schemas(CreateBlockRequest, Block)
    )
)]
pub struct ApiDoc;
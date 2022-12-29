use actix_web::web::to;
use serde_json::to_string;
use crate::route::auth::authentication::{__path_authentication, authentication};
use crate::route::registration::registration::{__path_registration, registration};
use crate::route::dto::auth_data::AuthData;
use crate::route::dto::auth_response::AuthResponse;
use crate::route::dto::error::ResponseError;
use utoipa::{ToSchema, Path, OpenApi};
use utoipa::openapi::{OpenApiBuilder, InfoBuilder, Info};

#[derive(OpenApi)]
#[openapi(
    paths(
        registration,
        authentication,
    ),
    components(
        schemas(
            AuthData,
            AuthResponse,
            ResponseError,
        ),
    ),
)]
pub struct ServiceApiDoc;

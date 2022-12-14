use crate::route::auth::authentication::__path_authentication;
use crate::route::registration::registration::__path_registration;
use crate::route::dto::auth_data::AuthData;
use crate::route::dto::auth_response::AuthResponse;

use crate::route::dto::error::ResponseError;
use utoipa::OpenApi;

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
        )
    ),
)]
pub struct ServiceApiDoc;

use crate::route::auth::authentication::__path_authentication;
use crate::route::registration::registration::__path_registration;
use crate::route::auth::model::AuthData;
use crate::route::registration::model::FormData;
use crate::route::auth::model::AuthResponse;

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
            FormData,
            AuthResponse,
        )
    ),
)]
pub struct ServiceApiDoc;

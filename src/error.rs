use crate::route::dto::error::ResponseError;
use actix_web::dev::ServiceResponse;
use actix_web::error::ErrorInternalServerError;
use actix_web::http::header::{ContentType, HeaderValue};
use actix_web::http::{header, StatusCode};
use actix_web::middleware::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::web::Bytes;
use actix_web::{dev, web, App, HttpResponse, Result};

pub fn error_to_json_handler<B: 'static>() -> ErrorHandlers<B> {
    ErrorHandlers::new()
        .handler(StatusCode::BAD_REQUEST, error_to_json)
        .handler(StatusCode::UNAUTHORIZED, error_to_json)
        .handler(StatusCode::PAYMENT_REQUIRED, error_to_json)
        .handler(StatusCode::FORBIDDEN, error_to_json)
        .handler(StatusCode::NOT_FOUND, error_to_json)
        .handler(StatusCode::METHOD_NOT_ALLOWED, error_to_json)
        .handler(StatusCode::NOT_ACCEPTABLE, error_to_json)
        .handler(StatusCode::PROXY_AUTHENTICATION_REQUIRED, error_to_json)
        .handler(StatusCode::REQUEST_TIMEOUT, error_to_json)
        .handler(StatusCode::CONFLICT, error_to_json)
        .handler(StatusCode::GONE, error_to_json)
        .handler(StatusCode::LENGTH_REQUIRED, error_to_json)
        .handler(StatusCode::PRECONDITION_FAILED, error_to_json)
        .handler(StatusCode::PAYLOAD_TOO_LARGE, error_to_json)
        .handler(StatusCode::URI_TOO_LONG, error_to_json)
        .handler(StatusCode::UNSUPPORTED_MEDIA_TYPE, error_to_json)
        .handler(StatusCode::RANGE_NOT_SATISFIABLE, error_to_json)
        .handler(StatusCode::EXPECTATION_FAILED, error_to_json)
        .handler(StatusCode::IM_A_TEAPOT, error_to_json)
        .handler(StatusCode::MISDIRECTED_REQUEST, error_to_json)
        .handler(StatusCode::UNPROCESSABLE_ENTITY, error_to_json)
        .handler(StatusCode::LOCKED, error_to_json)
        .handler(StatusCode::FAILED_DEPENDENCY, error_to_json)
        .handler(StatusCode::UPGRADE_REQUIRED, error_to_json)
        .handler(StatusCode::PRECONDITION_REQUIRED, error_to_json)
        .handler(StatusCode::TOO_MANY_REQUESTS, error_to_json)
        .handler(StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE, error_to_json)
        .handler(StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS, error_to_json)
        .handler(StatusCode::INTERNAL_SERVER_ERROR, error_to_json)
        .handler(StatusCode::NOT_IMPLEMENTED, error_to_json)
        .handler(StatusCode::BAD_GATEWAY, error_to_json)
        .handler(StatusCode::SERVICE_UNAVAILABLE, error_to_json)
        .handler(StatusCode::GATEWAY_TIMEOUT, error_to_json)
        .handler(StatusCode::HTTP_VERSION_NOT_SUPPORTED, error_to_json)
        .handler(StatusCode::VARIANT_ALSO_NEGOTIATES, error_to_json)
        .handler(StatusCode::INSUFFICIENT_STORAGE, error_to_json)
        .handler(StatusCode::LOOP_DETECTED, error_to_json)
        .handler(StatusCode::NOT_EXTENDED, error_to_json)
        .handler(StatusCode::NETWORK_AUTHENTICATION_REQUIRED, error_to_json)
}

pub fn error_to_json<B>(res: ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
    let (req, mut res) = res.into_parts();

    let header = match HeaderValue::from_str(ContentType::json().essence_str()) {
        Ok(hv) => hv,
        Err(_) => {
            return Err(ErrorInternalServerError(
                "ErrorResponse deserialization failed",
            ));
        }
    };
    res.headers_mut().insert(header::CONTENT_TYPE, header);

    let err_res = ResponseError {
        code: res.status().as_u16(),
        error: res.status().canonical_reason().map(|v| v.to_string()),
        message: res.error().map(|v| v.to_string())
    };

    let err_res = serde_json::to_string(&err_res);

    let error_resp = match err_res {
        Ok(err_res) => res.set_body(Bytes::from(err_res)),
        Err(_) => {
            return Err(ErrorInternalServerError(
                "ErrorResponse deserialization failed",
            ))
        }
    };

    let res = ServiceResponse::new(req, error_resp)
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res))
}

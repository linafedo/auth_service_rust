use actix_web::dev::always_ready;
use reqwest::{Client, Response};
use serde_json::Value;
use crate::lib::helpers::spawn_app;

const TEST_LOGIN: &str = "test_login";
const TEST_WRONG_LOGIN: &str = "test_login_wrong";
const TEST_PASSWORD: &str = "test_password";
const TEST_WRONG_PASSWORD: &str = "test_password_wrong";
const TEST_LOGIN_KEY: &str = "login";
const TEST_PASSWORD_KEY: &str = "password";

async fn get_response_for_auth(body: Value, client: Client, address: String) -> Response {
    return client
        .get(&format!("{}/authentication", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&body)
        .send()
        .await
        .expect("Failed to execute request.");
}

async fn get_response_for_registration(body: &Value, client: &Client, address: &String) -> Response {
    return client
        .post(&format!("{}/registration", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&body)
        .send()
        .await
        .expect("Failed to execute request.");
}

#[tokio::test]
async fn authentication_returns_a_200_for_valid_form_data() {
    // registration
    let test_data = spawn_app().await;
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        TEST_LOGIN_KEY: TEST_LOGIN,
        TEST_PASSWORD_KEY: TEST_PASSWORD
    });

    let response = get_response_for_registration(&body, &client, &test_data.address).await;
    assert_eq!(200, response.status().as_u16());

    // authentication
    let response = get_response_for_auth(body, client, test_data.address).await;
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT login FROM users")
        .fetch_one(&test_data.db_pool)
        .await
        .expect("Failed to fetch saved user.");

    assert_eq!(saved.login, TEST_LOGIN);
}

#[tokio::test]
async fn authentication_returns_a_409_for_invalid_password() {
    // registration
    let test_data = spawn_app().await;
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        TEST_LOGIN_KEY: TEST_LOGIN,
        TEST_PASSWORD_KEY: TEST_PASSWORD
    });

    let response = get_response_for_registration(&body, &client, &test_data.address).await;
    assert_eq!(200, response.status().as_u16());

    let body = serde_json::json!({
        TEST_LOGIN_KEY: TEST_LOGIN,
        TEST_PASSWORD_KEY: TEST_WRONG_PASSWORD
    });

    // authentication
    let response = get_response_for_auth(body, client, test_data.address).await;
    assert_eq!(409, response.status().as_u16());
}

#[tokio::test]
async fn authentication_returns_a_409_when_user_not_exist() {
    // registration
    let test_data = spawn_app().await;
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        TEST_LOGIN_KEY: TEST_LOGIN,
        TEST_PASSWORD_KEY: TEST_PASSWORD
    });

    let response = get_response_for_registration(&body, &client, &test_data.address).await;
    assert_eq!(200, response.status().as_u16());

    let body = serde_json::json!({
        TEST_LOGIN_KEY: TEST_WRONG_LOGIN,
        TEST_PASSWORD_KEY: TEST_WRONG_PASSWORD
    });

    // authentication
    let response = get_response_for_auth(body, client, test_data.address).await;
    assert_eq!(409, response.status().as_u16());
}
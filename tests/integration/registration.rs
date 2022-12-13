use reqwest::{Client, Response};
use serde_json::Value;
use auth_service::domain::user::new_user::PasswordData;
use crate::lib::helpers::spawn_app;

const TEST_LOGIN: &str = "user_login";
const TEST_PASSWORD: &str = "123456";
const TEST_WRONG_PASSWORD: &str = "123";
const TEST_LOGIN_KEY: &str = "login";
const TEST_PASSWORD_KEY: &str = "password";
const TEST_LOGIN_WRONG: &str = "a";

async fn get_response(body: Value, client: Client, address: String) -> Response {
    return client
        .post(&format!("{}/api/v1/registration", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&body)
        .send()
        .await
        .expect("Failed to execute request.");
}

#[tokio::test]
async fn registration_returns_a_200_for_valid_form_data() {
    let test_data = spawn_app().await;
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        TEST_LOGIN_KEY: TEST_LOGIN,
        TEST_PASSWORD_KEY: TEST_PASSWORD
    });

    let response = get_response(body, client, test_data.address).await;

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT login FROM users")
        .fetch_one(&test_data.db_pool)
        .await
        .expect("Failed to fetch saved user.");

    assert_eq!(saved.login, TEST_LOGIN);
}

#[tokio::test]
async fn registration_returns_a_400_when_login_is_wrong() {
    let test_data = spawn_app().await;
    let client = reqwest::Client::new();
    let body = serde_json::json!({ TEST_LOGIN_KEY: TEST_LOGIN_WRONG });

    let response = get_response(body, client, test_data.address).await;

    assert_eq!(
        400,
        response.status().as_u16()
    );
}

#[tokio::test]
async fn registration_returns_a_409_when_login_is_exist() {
    let test_data = spawn_app().await;
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        TEST_LOGIN_KEY: TEST_LOGIN,
        TEST_PASSWORD_KEY: TEST_PASSWORD,
    });

    let response = get_response(
        body.clone(),
        client.clone(),
        test_data.address.clone()
    ).await;

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT id, login, salt, password_hash FROM users")
        .fetch_one(&test_data.db_pool)
        .await
        .expect("Failed to fetch saved user.");

    assert_eq!(saved.login, TEST_LOGIN);

    let response = get_response(
        body,
        client,
        test_data.address
    ).await;

    assert_eq!(409, response.status().as_u16());
}

#[tokio::test]
async fn registration_returns_a_409_when_password_is_wrong() {
    let test_data = spawn_app().await;
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        TEST_LOGIN_KEY: TEST_LOGIN,
        TEST_PASSWORD_KEY: TEST_WRONG_PASSWORD,
    });

    let response = get_response(body, client, test_data.address).await;

    assert_eq!(409, response.status().as_u16());
}

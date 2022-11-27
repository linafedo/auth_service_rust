use crate::tests::spawn_app::spawn_app;
use reqwest::{Client, Response};
use serde_json::Value;

const TEST_LOGIN: &str = "user_login";
const TEST_PASSWORD: &str = "123456";
const TEST_WRONG_PASSWORD: &str = "123";
const TEST_LOGIN_KEY: &str = "login";
const TEST_PASSWORD_KEY: &str = "password";

async fn get_response(body: Value, client: Client, address: String) -> Response {
    return client
        .post(&format!("{}/registration", address))
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

    let saved = sqlx::query!("SELECT login, password FROM users")
        .fetch_one(&test_data.db_pool)
        .await
        .expect("Failed to fetch saved user.");

    assert_eq!(saved.login, TEST_LOGIN);
    assert_eq!(saved.password, TEST_PASSWORD);
}

#[tokio::test]
async fn registration_returns_a_400_when_data_is_missing() {
    let test_data = spawn_app().await;
    let client = reqwest::Client::new();
    let body = serde_json::json!({ TEST_LOGIN_KEY: TEST_LOGIN });

    let response = get_response(body, client, test_data.address).await;

    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn registration_returns_a_500_when_login_is_exist() {
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

    let saved = sqlx::query!("SELECT login, password FROM users")
        .fetch_one(&test_data.db_pool)
        .await
        .expect("Failed to fetch saved user.");

    assert_eq!(saved.login, TEST_LOGIN);
    assert_eq!(saved.password, TEST_PASSWORD);

    let response = get_response(
        body,
        client,
        test_data.address
    ).await;

    assert_eq!(500, response.status().as_u16());
}

#[tokio::test]
async fn registration_returns_a_500_when_password_is_wrong() {
    let test_data = spawn_app().await;
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        TEST_LOGIN_KEY: TEST_LOGIN,
        TEST_PASSWORD_KEY: TEST_WRONG_PASSWORD,
    });

    let response = get_response(body, client, test_data.address).await;

    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn authentication_returns_a_200_for_valid_form_data() {
    let test_data = spawn_app().await;
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        "login": "username",
        "password": "password"
    });
    println!("{}", body);
    let response = client
        .get(&format!("{}/authentication", test_data.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn authentication_returns_a_400_when_data_is_missing() {
    let test_data = spawn_app().await;
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        "login": "username",
    });

    let response = client
        .get(&format!("{}/authentication", test_data.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(
        400,
        response.status().as_u16()
    );
}
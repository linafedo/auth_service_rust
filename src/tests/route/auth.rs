use std::collections::HashMap;
use crate::tests::spawn_app::spawn_app;
use crate::bootstrap::Application;

#[tokio::test]
async fn registration_returns_a_200_for_valid_form_data() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        "login": "username",
        "password": "password"
    });
    println!("{}", body);
    let response = client
        .post(&format!("{}/registration", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn registration_returns_a_400_when_data_is_missing() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        "login": "username",
    });

    let response = client
        .post(&format!("{}/registration", address))
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

#[tokio::test]
async fn authentication_returns_a_200_for_valid_form_data() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        "login": "username",
        "password": "password"
    });
    println!("{}", body);
    let response = client
        .get(&format!("{}/authentication", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn authentication_returns_a_400_when_data_is_missing() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        "login": "username",
    });

    let response = client
        .get(&format!("{}/authentication", address))
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
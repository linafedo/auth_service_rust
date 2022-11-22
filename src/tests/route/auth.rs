use crate::tests::spawn_app::spawn_app;

#[tokio::test]
async fn registration_returns_a_200_for_valid_form_data() {
    let test_data = spawn_app().await;

    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "login": "user123",
        "password": "password123"
    });
    let response = client
        .post(&format!("{}/registration", test_data.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT login, password FROM users")
        .fetch_one(&test_data.db_pool)
        .await
        .expect("Failed to fetch saved user.");

    assert_eq!(saved.login, "user123");
    assert_eq!(saved.password, "password123");
}

#[tokio::test]
async fn registration_returns_a_400_when_data_is_missing() {
    let test_data = spawn_app().await;
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        "login": "username",
    });

    let response = client
        .post(&format!("{}/registration", test_data.address))
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
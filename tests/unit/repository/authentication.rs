const TEST_LOGIN: &str = "user_login";
const TEST_PASSWORD: &str = "123456";

#[cfg(test)]
mod auth_tests {
    use claim::assert_ok;
    use crate::lib::helpers::spawn_app;
    use auth_service::domain::user::new_user::NewUser;
    use auth_service::domain::user::user_data::{Password, Login, PasswordData};
    use crate::unit::repository::authentication::{TEST_LOGIN, TEST_PASSWORD};
    use auth_service::repository::password_data;
    use uuid::Uuid;
    use auth_service::repository::authentication::check_user;
    use auth_service::route::dto::auth_data::AuthData;

    #[tokio::test]
    async fn check_user_test() {
        let app = spawn_app().await;

        // Insert new user in database
        let login = Login::parse(TEST_LOGIN.to_string()).unwrap();
        let password = Password::parse(TEST_PASSWORD.to_string()).unwrap();
        let password_data = password_data::generate(TEST_PASSWORD).unwrap();
        let new_user = &NewUser::new(login, password, password_data);

        let result = sqlx::query!(
        r#"
            INSERT INTO users (id, login, salt, password_hash)
            VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        new_user.login.as_ref(),
        new_user.password_data.expose_salt_secret(),
        new_user.password_data.password_hash
        )
            .execute(app.db_pool.clone().get_ref())
            .await;
        assert_ok!(result);

        // Check user in database
        let auth_data = &AuthData{login:TEST_LOGIN.to_string(), password: TEST_PASSWORD.to_string()};
        let result = check_user(auth_data, app.db_pool.clone()).await;
        assert_eq!(result.unwrap().login, TEST_LOGIN.to_string());
    }
}
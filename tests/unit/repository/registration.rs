const TEST_LOGIN: &str = "user_login";
const TEST_PASSWORD: &str = "123456";

#[cfg(test)]
mod registration_tests {
    use claim::{assert_err, assert_ok};
    use auth_service::domain::user::new_user::NewUser;
    use auth_service::domain::user::user_data::Login;
    use auth_service::domain::user::user_data::Password;
    use auth_service::repository::password;
    use crate::lib::helpers::spawn_app;
    use auth_service::repository::registration::insert_user;
    use crate::unit::repository::registration::{TEST_LOGIN, TEST_PASSWORD};

    #[tokio::test]
    async fn insert_user_test() {
        let app = spawn_app().await;

        // Check for empty database
        let result = sqlx::query!(
             r#"
                SELECT id, login, password_hash, salt FROM users WHERE login = $1
            "#,
            TEST_LOGIN.to_string(),
        )
            .fetch_one(app.db_pool.clone().get_ref())
            .await;
        assert!(matches!(result.err().unwrap(), sqlx::Error::RowNotFound));

        let login = Login::parse(TEST_LOGIN.to_string()).unwrap();
        let password = Password::parse(TEST_PASSWORD.to_string()).unwrap();
        let password_data = password::generate(TEST_PASSWORD).unwrap();
        let new_user = &NewUser::new(login, password, password_data);

        // Insert new user in database
        let result = insert_user(new_user, app.db_pool.clone()).await;
        assert_ok!(result);

        // Check added user in database
        let result = sqlx::query!(
             r#"
                SELECT id, login, password_hash, salt FROM users WHERE login = $1
            "#,
            TEST_LOGIN.to_string(),
        )
            .fetch_one(app.db_pool.clone().get_ref())
            .await;
        assert_ok!(result);
    }
}
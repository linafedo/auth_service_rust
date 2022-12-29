#[cfg(test)]
mod password_manager_tests {
    use auth_service::repository::password_data::password::{check_password, generate};
    use auth_service::domain::user::user_data::Password;
    use secrecy::{ExposeSecret, Secret};
    use claim::{assert_ok, assert_some};
    use auth_service::repository::password_data::error::Error;

    #[test]
    fn check_password_success() {
        let password_str = Secret::new("123456".to_string());
        let password_data = generate(password_str.clone()).unwrap();

        let result = check_password(
            password_str.clone(),
            password_data.salt.clone(),
            password_data.password_hash.clone()
        );
        assert_ok!(result);
    }

    #[test]
    fn check_password_fail() {
        let password_str = Secret::new("123456".to_string());
        let password = Password::parse(password_str.clone()).unwrap();
        let password_data = generate(password_str.clone()).unwrap();

        let wrong_password = Secret::new("111111".to_string());
        let result = check_password(
            wrong_password.clone(),
            password_data.salt.clone(),
            password_data.password_hash.clone()
        );
        assert!(matches!(result.err().unwrap(), Error::PasswordNotCorrect));

    }

    #[test]
    fn generate_password_data () {
        let password_str = Secret::new("123456".to_string());
        let password = Password::parse(password_str).unwrap();
        let password_data = generate(password.0);

        assert_ok!(password_data);
    }
}
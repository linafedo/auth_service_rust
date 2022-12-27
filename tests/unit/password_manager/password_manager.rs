#[cfg(test)]
mod password_manager_tests {
    use auth_service::password_manager::manager;
    use auth_service::domain::user::user_data::Password;
    use secrecy::ExposeSecret;
    use claim::{assert_ok, assert_some};
    use auth_service::domain::user::error::DomainError;

    #[test]
    fn check_password_success() {
        let password_str = "123456";
        let password = Password::parse(password_str.to_string()).unwrap();
        let password_data = manager::generate(password.expose_secret()).unwrap();

        let result = manager::check_password(
            &password_str,
            password_data.salt.expose_secret().as_str(),
            password_data.password_hash.as_str()
        );
        assert_ok!(result);
    }

    #[test]
    fn check_password_fail() {
        let password_str = "123456";
        let password = Password::parse(password_str.to_string()).unwrap();
        let password_data = manager::generate(password.expose_secret()).unwrap();

        let wrong_password = "111111".to_string();
        let result = manager::check_password(
            &wrong_password,
            password_data.salt.expose_secret().as_str(),
            password_data.password_hash.as_str()
        );
        assert_eq!(result.err().unwrap(), DomainError::PasswordNotCorrect);
    }

    #[test]
    fn generate_password_data () {
        let password_str = "123456";
        let password = Password::parse(password_str.to_string()).unwrap();
        let password_data = manager::generate(password.expose_secret());

        assert_ok!(password_data);
    }
}
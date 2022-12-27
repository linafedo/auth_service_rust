#[cfg(test)]
mod login_tests {
    use claim::{assert_ok};
    use auth_service::domain::user::error::DomainError;
    use auth_service::domain::user::user_data::Login;

    #[test]
    fn login_length_is_valid() {
        let name = "a".repeat(256);
        assert_ok!( Login::parse(name));
    }

    #[test]
    fn login_length_is_wrong() {
        let items = ["a".repeat(258), "aa".to_string(), "a".to_string()];
        for i in items {
            let result = Login::parse(i);
            assert_eq!( result.err().unwrap(), DomainError::LoginLengthIsWrong);
        }
    }

    #[test]
    fn login_contains_forbidden_characters() {
        let items = [
            "(Alex)",
            "/Alex",
            "\"Alex",
            "<Alex>",
            "{Alex}",
            "-Alex-",
            "\\Alex-",
        ];
        for i in items {
            let result = Login::parse(i.to_string());
            assert_eq!( result.err().unwrap(), DomainError::LoginIsNotCorrect);
        }
    }

    #[test]
    fn login_is_empty() {
        let items = ["", " "];
        for i in items {
            let result = Login::parse(i.to_string());
            assert_eq!( result.err().unwrap(), DomainError::LoginIsEmpty);
        }
    }
}

#[cfg(test)]
mod password_tests {
    use claim::assert_ok;
    use auth_service::domain::user::user_data::{Login, Password};
    use auth_service::domain::user::error::DomainError;
    use auth_service::domain::user::new_user::NewUser;
    use auth_service::password_manager::manager;

    #[test]
    fn password_is_correct() {
        let password = "a".repeat(256);
        assert_ok!( Password::parse(password));
    }

    #[test]
    fn password_is_not_correct() {
        let items = [
            "".to_string(),
            "a".repeat(5),
            "a".repeat(257)
        ];
        for i in items {
            let result = Password::parse(i);
            assert_eq!( result.err().unwrap(), DomainError::PasswordNotCorrect);
        }
    }

    #[test]
    fn return_valid_password_fields() {
        let login = Login::parse("Alex".to_string()).unwrap();
        let password_str = "123456";
        let password = Password::parse(password_str.to_string()).unwrap();
        let password2 = Password::parse(password_str.to_string()).unwrap();
        let password_data = manager::generate(password.expose_secret()).unwrap();

        let user = NewUser::new(login, password, password_data);

        assert_eq!(user.password.as_ref(), password2.as_ref());
        assert_eq!(user.password.expose_secret(), password2.expose_secret());
    }
}

#[cfg(test)]
mod password_data_tests {
    use claim::assert_ok;
    use secrecy::ExposeSecret;
    use auth_service::domain::user::user_data::{Login, Password};
    use auth_service::domain::user::error::DomainError;
    use auth_service::domain::user::new_user::NewUser;
    use auth_service::password_manager::manager;

    #[test]
    fn return_valid_password_data_fields() {
        let login = Login::parse("Alex".to_string()).unwrap();
        let password = Password::parse( "123456".to_string()).unwrap();
        let password_data = manager::generate(password.expose_secret()).unwrap();
        let password_data_clone = password_data.clone();
        let user = NewUser::new(login, password, password_data);

        assert_eq!(user.password_data.password_hash, password_data_clone.password_hash);
        assert_eq!(user.password_data.salt.expose_secret(), password_data_clone.salt.expose_secret());
    }
}
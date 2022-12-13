#[cfg(test)]
mod login_tests {
    use claim::{assert_ok};
    use auth_service::domain::user::error::DomainError;
    use auth_service::domain::user::new_user::UserLogin;

    #[test]
    fn login_length_is_valid() {
        let name = "a".repeat(256);
        assert_ok!( UserLogin::parse(name));
    }

    #[test]
    fn login_length_is_wrong() {
        let items = ["a".repeat(258), "aa".to_string(), "a".to_string()];
        for i in items {
            let result = UserLogin::parse(i);
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
            let result = UserLogin::parse(i.to_string());
            assert_eq!( result.err().unwrap(), DomainError::LoginIsNotCorrect);
        }
    }

    #[test]
    fn login_is_empty() {
        let items = ["", " "];
        for i in items {
            let result = UserLogin::parse(i.to_string());
            assert_eq!( result.err().unwrap(), DomainError::LoginIsEmpty);
        }
    }
}

#[cfg(test)]
mod password_tests {
    use claim::{assert_err, assert_ok};
    use auth_service::domain::user::new_user::{UserLogin, UserPassword, PasswordData};
    use auth_service::domain::user::error::DomainError;
    use auth_service::route::dto::auth_data::NewUser;

    #[test]
    fn password_is_correct() {
        let password = "a".repeat(256);
        assert_ok!( UserPassword::parse(password));
    }

    #[test]
    fn password_is_not_correct() {
        let items = [
            "".to_string(),
            "a".repeat(5),
            "a".repeat(257)
        ];
        for i in items {
            let result = UserPassword::parse(i);
            assert_eq!( result.err().unwrap(), DomainError::PasswordNotCorrect);
        }
    }

    #[test]
    fn return_valid_password_fields() {
        let login = UserLogin::parse("Alex".to_string()).unwrap();
        let password_str = "123456";
        let password = UserPassword::parse(password_str.to_string()).unwrap();
        let password2 = UserPassword::parse(password_str.to_string()).unwrap();
        let password_data = PasswordData::generate(password.expose_secret()).unwrap();

        let user = NewUser::new(login, password, password_data);

        assert_eq!(user.password.as_ref(), password2.as_ref());
        assert_eq!(user.password.expose_secret(), password2.expose_secret());
    }
}

#[cfg(test)]
mod password_data_tests {
    use claim::{assert_err, assert_ok};
    use auth_service::domain::user::new_user::{UserLogin, UserPassword, PasswordData};
    use auth_service::domain::user::error::DomainError;
    use auth_service::route::dto::auth_data::NewUser;

    #[test]
    fn return_valid_password_data_fields() {
        let login = UserLogin::parse("Alex".to_string()).unwrap();
        let password = UserPassword::parse( "123456".to_string()).unwrap();
        let password_data = PasswordData::generate(password.expose_secret()).unwrap();
        let password_data_clone = password_data.clone();

        let user = NewUser::new(login, password, password_data);

        assert_eq!(user.password_data.get_password_hash(), password_data_clone.get_password_hash());
        assert_eq!(user.password_data.get_salt(), password_data_clone.get_salt());
    }

    #[test]
    fn check_password_success() {
        let login = UserLogin::parse("Alex".to_string()).unwrap();
        let password_str = "123456";
        let password = UserPassword::parse( password_str.to_string()).unwrap();
        let password_data = PasswordData::generate(password.expose_secret()).unwrap();
        let password_data_clone = password_data.clone();

        let user = NewUser::new(login, password, password_data);

        let result = PasswordData::check_password(
            &password_str,
            password_data_clone.get_salt(),
            password_data_clone.get_password_hash()
        );
        assert_ok!(result);
    }

    #[test]
    fn check_password_fail() {
        let login = UserLogin::parse("Alex".to_string()).unwrap();
        let password_str = "123456";
        let password = UserPassword::parse( password_str.to_string()).unwrap();
        let password_data = PasswordData::generate(password.expose_secret()).unwrap();
        let password_data_clone = password_data.clone();

        let user = NewUser::new(login, password, password_data);

        let wrong_password = "111111".to_string();
        let result = PasswordData::check_password(
            &wrong_password,
            password_data_clone.get_salt(),
            password_data_clone.get_password_hash()
        );
        assert_eq!( result.err().unwrap(), DomainError::PasswordNotCorrect);
    }
}
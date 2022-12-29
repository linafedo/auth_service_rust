#[cfg(test)]
mod login_tests {
    use claim::{assert_ok};
    use secrecy::Secret;
    use auth_service::domain::user::error::Error;
    use auth_service::domain::user::user_data::Login;

    #[test]
    fn login_length_is_valid() {
        let name = "a".repeat(256);
        assert_ok!( Login::parse(Secret::new(name)));
    }

    #[test]
    fn login_length_is_wrong() {
        let items = ["a".repeat(258), "aa".to_string(), "a".to_string()];
        for i in items {
            let result = Login::parse(Secret::new(i));
            assert_eq!( result.err().unwrap(), Error::LoginLengthIsWrong);
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
            let result = Login::parse(Secret::new(i.to_string()));
            assert_eq!( result.err().unwrap(), Error::LoginIsNotCorrect);
        }
    }

    #[test]
    fn login_is_empty() {
        let items = ["", " "];
        for i in items {
            let result = Login::parse(Secret::new(i.to_string()));
            assert_eq!( result.err().unwrap(), Error::LoginLengthIsWrong);
        }
    }
}

#[cfg(test)]
mod password_tests {
    use actix_web::error::UrlencodedError::Serialize;
    use claim::assert_ok;
    use secrecy::{ExposeSecret, Secret};
    use auth_service::domain::user::user_data::{Login, Password};
    use auth_service::domain::user::error::Error;
    use auth_service::domain::user::new_user::NewUser;
    use auth_service::repository::password_data::password::generate;

    #[test]
    fn password_is_correct() {
        let password = "a".repeat(256);
        assert_ok!( Password::parse(Secret::new(password)));
    }

    #[test]
    fn password_is_not_correct() {
        let items = [
            "".to_string(),
            "a".repeat(5),
            "a".repeat(257)
        ];
        for i in items {
            let result = Password::parse(Secret::new(i));
            assert_eq!( result.err().unwrap(), Error::PasswordNotCorrect);
        }
    }

    #[test]
    fn return_valid_password_fields() {
        let login = Login::parse(Secret::new("Alex".to_string())).unwrap();
        let password_str = "123456";
        let password = Password::parse(
            Secret::new(password_str.to_string())
        )
            .unwrap();
        let password2 = Password::parse(
            Secret::new(password_str.to_string())
        )
            .unwrap();
        let password_data = generate(
            Secret::new(password_str.to_string())
        )
            .unwrap();

        let user = NewUser::new(login.clone(), password.clone(), password_data);

        assert_eq!(user.password.0.expose_secret(), password.0.expose_secret());
        assert_eq!(user.login.0.expose_secret(), login.0.expose_secret());


    }
}

#[cfg(test)]
mod password_data_tests {
    use claim::assert_ok;
    use secrecy::{ExposeSecret, Secret};
    use auth_service::domain::user::user_data::{Login, Password};
    use auth_service::domain::user::new_user::NewUser;
    use auth_service::repository::password_data::password::generate;

    #[test]
    fn return_valid_password_data_fields() {
        let login = Login::parse(Secret::new("Alex".to_string())).unwrap();
        let password = Password::parse(
            Secret::new("123456".to_string())
        ).unwrap();
        let password_data = generate(
            Secret::new("123456".to_string())
        ).unwrap();
        let password_data_clone = password_data.clone();
        let user = NewUser::new(login, password, password_data);

        assert_eq!(
            user.password_data.password_hash.expose_secret(),
            password_data_clone.password_hash.expose_secret()
        );
        assert_eq!(
            user.password_data.salt.expose_secret(),
            password_data_clone.salt.expose_secret()
        );
    }
}
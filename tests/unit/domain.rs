#[cfg(test)]
mod login_tests {
    use claim::{assert_err, assert_ok};
    use auth_service::domain::user::error::DomainError;
    use auth_service::domain::user::model::UserLogin;

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
    use auth_service::domain::user::model::UserPassword;

    #[test]
    fn password_is_correct() {
        let password = "a".repeat(25);
        assert_ok!( UserPassword::parse(password));
    }

    #[test]
    fn password_is_empty() {
        assert_err!( UserPassword::parse("".to_string()));
    }

    #[test]
    fn password_length_is_long() {
        let password = "a".repeat(257);
        assert_err!( UserPassword::parse(password));
    }

    #[test]
    fn password_length_is_short() {
        let password = "a".repeat(5);
        assert_err!( UserPassword::parse(password));
    }
}
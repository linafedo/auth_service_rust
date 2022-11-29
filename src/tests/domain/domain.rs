use crate::route::domain::{UserLogin, UserPassword};
use claim;

#[cfg(test)]
mod tests {
    use claim::{assert_err, assert_ok};
    use crate::route::domain::UserLogin;

    #[test]
    fn login_length_is_valid() {
        let name = "a".repeat(256);
        assert_ok!( UserLogin::parse(name));
    }

    #[test]
    fn login_length_is_long() {
        let name = "a".repeat(258);
        assert_err!( UserLogin::parse(name));
    }

    #[test]
    fn login_is_empty() {
        assert_err!( UserLogin::parse("".to_string()));
    }

    #[test]
    fn login_contains_forbidden_characters() {
        let name = "(Alex)".to_string();
        assert_err!( UserLogin::parse(name));

        let name = "/Alex".to_string();
        assert_err!( UserLogin::parse(name));

        let name = "\"Alex".to_string();
        assert_err!( UserLogin::parse(name));

        let name = "<Alex>".to_string();
        assert_err!( UserLogin::parse(name));

        let name = "{Alex}".to_string();
        assert_err!( UserLogin::parse(name));

        let name = "-Alex-".to_string();
        assert_err!( UserLogin::parse(name));

        let name = "\\Alex-".to_string();
        assert_err!( UserLogin::parse(name));
    }

    #[test]
    fn login_contains_whitespace() {
        let name = " ".to_string();
        assert_err!( UserLogin::parse(name));
    }
}
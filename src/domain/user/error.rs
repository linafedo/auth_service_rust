
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("Login length is wrong.")]
    LoginLengthIsWrong,
    #[error("Received login is not correct.")]
    LoginIsNotCorrect,
    #[error("Password is not correct.")]
    PasswordNotCorrect
}
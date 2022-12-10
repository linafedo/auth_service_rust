
#[derive(Clone, Copy, thiserror::Error, Debug, PartialEq)]
pub enum DomainError {
    #[error("Received login is empty.")]
    LoginIsEmpty,
    #[error("Login length is wrong.")]
    LoginLengthIsWrong,
    #[error("Received login is not correct.")]
    LoginIsNotCorrect,
    #[error("Password is not correct.")]
    PasswordNotCorrect,
    #[error("Something went wrong during hashing.")]
    HashingError
}
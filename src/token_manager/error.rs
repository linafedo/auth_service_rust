pub enum TokenError {
    WrongFileLength,
    DecodeSecretError(String),
    FileWithSecretNotFound(String),
    CreateFileForSecretError(String),
    WriteSecretToFileError(String),
    GenerateKeyError(String),
    SignTokenError(String),
    VerifyTokenError,
}
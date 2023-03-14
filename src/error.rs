#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    RandError(#[from] rand::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error)
}

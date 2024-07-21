#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    DatabaseError(#[from] diesel::result::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}

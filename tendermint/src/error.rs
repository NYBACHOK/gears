use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Genesis(#[from] serde_json::Error),

    #[error("invalid data: {0}")]
    InvalidData(String),
}

pub mod proto {
    pub use tendermint_proto::Error;
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DockerAPIError {
    #[error(
        "invalid api response message {message:?}, status {status_code:?}"
    )]
    InvalidApiResponse { status_code: String, message: String },

    #[error("failed to send request")]
    HyperError(#[from] hyper::Error),

    #[error("failed to serialize model")]
    SerializeError(#[from] serde_json::Error),
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DockerAPIError {
    #[error("invalid api response message {message:?}, status {status_code:?}")]
    InvalidApiResponse {
        status_code: String,
        message: String
    },
    #[error("unknown error")]
    OtherError(#[from] anyhow::Error)
}
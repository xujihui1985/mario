use hyper::{body::Buf, Client};
use hyperlocal::{UnixClientExt, UnixConnector, Uri};

use crate::connector::models::{Container, ContainerDetail};

use super::errors::DockerAPIError;

const INFO_PATH: &'static str = "/info";
const LIST_CONTAINERS_PATH: &'static str = "/containers/json";

pub struct DockerClient {
    sock_path: String,
    client: Client<UnixConnector>,
}

pub type DockerAPIResult<T> = Result<T, DockerAPIError>;

impl DockerClient {
    pub fn connect<T: Into<String>>(p: T) -> Self {
        let unix = Client::unix();
        let client = DockerClient { sock_path: p.into(), client: unix };
        client
    }

    pub async fn list_containers(&self) -> DockerAPIResult<Vec<Container>> {
        let url = Uri::new(&self.sock_path, LIST_CONTAINERS_PATH).into();
        let response = self.client.get(url).await?;
        let status = response.status();
        if !status.is_success() {
            return Err(DockerAPIError::InvalidApiResponse {
                message: "failed to list containers".to_string(),
                status_code: status.to_string(),
            });
        }
        let body = hyper::body::aggregate(response).await?;
        let containers = serde_json::from_reader(body.reader())?;
        Ok(containers)
    }

    pub async fn info(&self) -> DockerAPIResult<bool> {
        let url = Uri::new(&self.sock_path, INFO_PATH).into();
        let response = self.client.get(url).await?;
        let status = response.status();
        Ok(status.is_success())
    }

    pub async fn inspect(
        &self,
        id: impl Into<String>,
    ) -> DockerAPIResult<ContainerDetail> {
        let url = Uri::new(
            &self.sock_path,
            &format!("/containers/{}/json", id.into()),
        )
        .into();
        let response = self.client.get(url).await.unwrap();
        let status = response.status();
        if !status.is_success() {
            return Err(DockerAPIError::InvalidApiResponse {
                message: "failed to inspect container".to_string(),
                status_code: status.to_string(),
            });
        }
        let body = hyper::body::aggregate(response).await?;
        let detail: ContainerDetail = serde_json::from_reader(body.reader())?;
        Ok(detail)
    }
}

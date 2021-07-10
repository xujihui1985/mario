use hyper::{Client};
use hyperlocal::{UnixClientExt, UnixConnector, Uri};

const INFO_PATH: &'static str = "/info";

pub struct DockerClient {
    sock_path: String,
    client: Client<UnixConnector>,
}

impl DockerClient {
    pub fn connect<T: Into<String>>(p: T) -> Self {
        let unix = Client::unix();
        let client = DockerClient {
            sock_path: p.into(),
            client: unix,
        };
        client
    }

    pub fn list_containers(&self) {}

    pub async fn info(&self) -> bool {
        let url = Uri::new(&self.sock_path, INFO_PATH).into();
        let response = self.client.get(url).await.unwrap();
        let status = response.status();
        status.is_success()
    }
}

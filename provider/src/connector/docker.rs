use hyper::{body::Buf, Client};
use hyperlocal::{UnixClientExt, UnixConnector, Uri};
use serde::{Deserialize, Serialize};

const INFO_PATH: &'static str = "/info";
const LIST_CONTAINERS_PATH: &'static str = "/containers/json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Names")]
    pub names: Vec<String>,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "Status")]
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerDetail {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "State")]
    pub state: ContainerState,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ContainerState {
    #[serde(rename = "Pid")]
    pub pid: u64,
}

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

    pub async fn list_containers(&self) {
        let url = Uri::new(&self.sock_path, LIST_CONTAINERS_PATH).into();
        let response = self.client.get(url).await.unwrap();
        let status = response.status();
        if status.is_success() {
            let body = hyper::body::aggregate(response).await.unwrap();
            let containers: Vec<Container> = serde_json::from_reader(body.reader()).unwrap();
            println!("{:?}", containers);
        }
    }

    pub async fn info(&self) -> bool {
        let url = Uri::new(&self.sock_path, INFO_PATH).into();
        let response = self.client.get(url).await.unwrap();
        let status = response.status();
        status.is_success()
    }

    pub async fn inspect(&self, id: impl Into<String>) {
        let url = Uri::new(&self.sock_path, &format!("/containers/{}/json", id.into())).into();
        let response = self.client.get(url).await.unwrap();
        let status = response.status();
        if status.is_success() {
            let body = hyper::body::aggregate(response).await.unwrap();
            let detail: ContainerDetail = serde_json::from_reader(body.reader()).unwrap();
            println!("{:?}", detail);
        }
    }
}

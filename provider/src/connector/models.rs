use serde::{Deserialize, Serialize};

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

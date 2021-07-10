use provider::connector::docker::DockerClient;

#[tokio::main]
async fn main() {
    println!("before connect");
    {
        let client = DockerClient::connect("/var/run/docker.sock");
        client.inspect("ed8f5b69e107").await;
    }
    println!("after connect");
}

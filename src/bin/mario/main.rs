use provider::connector::docker::DockerClient;

#[tokio::main]
async fn main() {
    println!("before connect");
    std::thread::sleep(std::time::Duration::from_secs(30));
    {
        let client = DockerClient::connect("/var/run/docker.sock");
        client.info().await;
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
    println!("after connect");
    std::thread::sleep(std::time::Duration::from_secs(60));
}

use std::collections::HashMap;
use tokio::sync::oneshot;

use provider::connector::{
    docker::DockerClient,
    models::{Container, ContainerDetail},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DockerClient::connect("/var/run/docker.sock");
    let container_list = client.list_containers().await?;
    let containers: HashMap<String, Container> = container_list
        .into_iter()
        .map(|c| (c.id.clone(), c))
        .collect();
    let (tx, stop) = oneshot::channel::<()>();
    let watch_container_handler = tokio::spawn(async move {
        tokio::select! {
            _ = task_inner(&client) => { },
            _ = stop => {
                println!("stop");
            }
        }
    });
    let timeout_handle = tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(12)).await;
        tx.send(()).unwrap();
    });
    watch_container_handler.await?;
    timeout_handle.await?;
    Ok(())
}

async fn task_inner(client: &DockerClient) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
    loop {
        interval.tick().await;
        let list_container_result = client.list_containers().await;
        match list_container_result {
            Ok(containers) => {
                let container_details: Vec<_> = containers
                    .iter()
                    .map(|c| client.inspect(c.id.clone()))
                    .collect();
                let mut res = Vec::<ContainerDetail>::new();
                for task in container_details {
                    match task.await {
                        Ok(detail) => res.push(detail),
                        Err(e) => println!("failed to inspect container {:?}", e),
                    }
                }
                println!("list containers {:?}", res);
            }
            Err(e) => {
                println!("failed to listcontainer {:?}", e);
            }
        }
    }
}

async fn collect(containers: HashMap<String, Container>) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
    loop {
        interval.tick().await;
        containers.iter().for_each(|(id, c)| {});
    }
}

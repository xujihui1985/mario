use std::collections::HashMap;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    sync::oneshot,
};

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
    let collect_handler = tokio::spawn(collect());
    let timeout_handle = tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(12)).await;
        tx.send(()).unwrap();
    });
    watch_container_handler.await?;
    timeout_handle.await?;
    collect_handler.await?;
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

async fn collect() {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
    loop {
        interval.tick().await;
        let proc_stat = tokio::fs::File::open("/proc/stat").await.unwrap();
        let buf_reader = BufReader::new(proc_stat);
        let mut lines = buf_reader.lines();
        while let Some(line) = lines.next_line().await.unwrap() {
            if line.starts_with("cpu ") {
                let part = line[5..].split(" ").collect::<Vec<_>>();
                println!("usr {}, nice {}, sys {}, idle {}, iowait {}, hardirq {}, softirq {}, steal {}, guest {}", part[0], part[1], part[2], part[3], part[4], part[5], part[6], part[7], part[8]);
                break;
            }
        }

        let cpuinfo = tokio::fs::File::open("/proc/cpuinfo").await.unwrap();
        let buf_reader = BufReader::new(cpuinfo);
        let mut lines = buf_reader.lines();
        let mut number_of_cpus = 0;
        while let Some(line) = lines.next_line().await.unwrap() {
            if line.starts_with("processor\t:") {
                number_of_cpus = number_of_cpus + 1;
            }
        }
        println!("number of cpu {}", number_of_cpus);
    }
}

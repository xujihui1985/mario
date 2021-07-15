use collector::{self, cpu};
use futures::future::join_all;
use storage;
use tokio::sync::broadcast;

use provider::connector::docker::DockerClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let container_list = client.list_containers().await?;
    // let containers: HashMap<String, Container> =
    //     container_list.into_iter().map(|c| (c.id.clone(), c)).collect();
    let (tx, stop) = broadcast::channel::<()>(2);
    let rx2 = tx.subscribe();

    let watch_container_handle = tokio::spawn(watch_containers(
        DockerClient::connect("/var/run/pouchd.sock"),
        stop,
    ));

    let cpu_collector =
        collector::cpu::CPUCollector { name: "cpu".to_string() };
    let mem_collector =
        collector::mem::MemCollector { name: "mem".to_string() };
    let mut collectors = Vec::<Box<dyn storage::Collector>>::new();
    collectors.push(Box::new(cpu_collector));
    collectors.push(Box::new(mem_collector));
    let collect_handle = tokio::spawn(collect(5, collectors, rx2));
    let timeout_handle = tokio::spawn(sleep(tx)); // tokio spawn must be a future
    watch_container_handle.await?;
    timeout_handle.await?;
    collect_handle.await?;
    Ok(())
}

async fn watch_containers(
    client: DockerClient,
    mut stop: tokio::sync::broadcast::Receiver<()>,
) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
    loop {
        tokio::select! {
            _ = interval.tick() => {
                let list_container_result = client.list_containers().await;
                match list_container_result {
                    Ok(containers) => {
                        let container_details: Vec<_> = containers
                            .iter()
                            .map(|c| client.inspect(c.id.clone()))
                            .collect();
                        let result: Vec<_> = join_all(container_details)
                            .await
                            .into_iter()
                            .filter_map(|x| x.ok())
                            .collect();
                        println!("list containers {:?}", result);
                    }
                    Err(e) => {
                        println!("failed to listcontainer {:?}", e);
                    }
                }
            },
            _ = stop.recv() => {
                println!("stop");
                break;
            }
        }
    }
}

async fn sleep(tx: tokio::sync::broadcast::Sender<()>) {
    tokio::time::sleep(std::time::Duration::from_secs(12)).await;
    tx.send(()).unwrap();
}

async fn collect(
    interval_seconds: u64,
    collectors: Vec<Box<dyn storage::Collector>>,
    mut stop: tokio::sync::broadcast::Receiver<()>,
) {
    let mut interval =
        tokio::time::interval(std::time::Duration::from_secs(interval_seconds));
    loop {
        tokio::select! {
            _ = interval.tick() => {
                for collector in &collectors {
                    println!("{}", collector.get_name());
                    collector.collect().await;
                }
            }
            _ = stop.recv() => {
                println!("stop collect");
                break;
            }
        }
    }
}

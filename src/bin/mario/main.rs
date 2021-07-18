use futures::future::join_all;
use rocksdb::{WriteBatch, DB};
use std::sync::Arc;
use tokio::sync::broadcast;

use provider::connector::docker::DockerClient;

struct App {
    interval_seconds: u64,
    collectors: Vec<Box<dyn mario_core::Collector>>,
    db: Arc<rocksdb::DBWithThreadMode<rocksdb::MultiThreaded>>,
}

impl App {
    fn new(db: rocksdb::DBWithThreadMode<rocksdb::MultiThreaded>) -> Self {
        let cpu_collector = collector::cpu::CPUCollector::new();
        let mem_collector = collector::mem::MemCollector::new();
        let mut collectors = Vec::<Box<dyn mario_core::Collector>>::new();
        collectors.push(Box::new(cpu_collector));
        collectors.push(Box::new(mem_collector));

        App { collectors: collectors, interval_seconds: 5, db: Arc::new(db) }
    }

    async fn collect(&self, mut stop: tokio::sync::broadcast::Receiver<()>) {
        let mut interval = tokio::time::interval(
            std::time::Duration::from_secs(self.interval_seconds),
        );
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    let mut batch = WriteBatch::default();
                    for clt in &self.collectors {
                        clt.collect(self.db.clone(), &mut batch).await;
                    }
                    self.db.write(batch).unwrap();
                    println!("save stats");
                }
                _ = stop.recv() => {
                    break;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let container_list = client.list_containers().await?;
    // let containers: HashMap<String, Container> =
    //     container_list.into_iter().map(|c| (c.id.clone(), c)).collect();
    let (tx, stop) = broadcast::channel::<()>(2);
    let rx2 = tx.subscribe();

    let watch_container_handle = tokio::spawn(watch_containers(
        DockerClient::connect("/var/run/docker.sock"),
        stop,
    ));

    let mut opt = rocksdb::Options::default();
    opt.set_db_write_buffer_size(128 << 20);
    opt.create_missing_column_families(true);
    opt.create_if_missing(true);

    let db = rocksdb::DBWithThreadMode::<rocksdb::MultiThreaded>::open_cf(
        &opt,
        "testpath",
        &["cpu", "mem"],
    )?;
    let app = App::new(db);
    let arc_app = Arc::new(app);
    let arc_app = arc_app.clone();
    tokio::spawn(async move {
        arc_app.collect(rx2).await;
    });

    // let collect_handle = tokio::spawn(collect(5, collectors, rx2));
    let timeout_handle = tokio::spawn(sleep(tx)); // tokio spawn must be a future
    watch_container_handle.await?;
    timeout_handle.await?;
    // collect_handle.await?;
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

// async fn collect(
//     interval_seconds: u64,
//     collectors: Vec<Box<dyn mario_core::Collector>>,
//     mut stop: tokio::sync::broadcast::Receiver<()>,
// ) {
//     let mut interval =
//         tokio::time::interval(std::time::Duration::from_secs(interval_seconds));
//     loop {
//         tokio::select! {
//             _ = interval.tick() => {
//                 let mut batch = WriteBatch::default();
//                 batch.put_cf(cf, key, value)
//                 for clt in &collectors {
//                     let cf = db.cf_handle(clt.get_name()).unwrap();
//                     clt.collect(|k, v| {
//                         batch.put_cf(cf, k, v);
//                     }).await;
//                 }
//                 db.write(batch);
//             }
//             _ = stop.recv() => {
//                 println!("stop collect");
//                 break;
//             }
//         }
//     }
// }

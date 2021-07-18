use std::{
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

pub trait Collector: Sync + Send {
    fn get_name(&self) -> String;

    fn collect(
        &self,
        db: Arc<rocksdb::DBWithThreadMode<rocksdb::MultiThreaded>>,
        batch: &mut rocksdb::WriteBatch,
    );
}

pub struct WriteBatchWithColumeFamily<'a> {
    pub batch: Arc<Mutex<rocksdb::WriteBatch>>,
    pub cf: rocksdb::BoundColumnFamily<'a>,
}

pub fn get_current_timestamp() -> u64 {
    let start = SystemTime::now();
    start.duration_since(UNIX_EPOCH).unwrap().as_secs()
}

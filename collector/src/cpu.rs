use serde::Serialize;
use std::{io::{BufRead, BufReader}, sync::Arc};

use mario_core::Collector;

pub struct CPUCollector {
    pub name: String,
    // pub fields: Vec<Field>,
}

#[derive(Debug, Default, Serialize)]
pub struct CPUStat {
    usr: u64,
    nice: u64,
    number_of_cpus: u32,
}

impl CPUCollector {
    pub fn new() -> Self {
        CPUCollector { name: String::from("cpu") }
    }
}

impl Collector for CPUCollector {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn collect(
        &self,
        db: Arc<rocksdb::DBWithThreadMode<rocksdb::MultiThreaded>>,
        batch: &mut rocksdb::WriteBatch,
    ) {
        let proc_stat = std::fs::File::open("/proc/stat").unwrap();
        let buf_reader = BufReader::new(proc_stat);

        let mut stat = CPUStat::default();
        for line in buf_reader.lines() {
            let line = line.unwrap();
            if line.starts_with("cpu ") {
                let part = line[5..].split(" ").collect::<Vec<_>>();
                println!("usr {}, nice {}, sys {}, idle {}, iowait {}, hardirq {}, softirq {}, steal {}, guest {}", part[0], part[1], part[2], part[3], part[4], part[5], part[6], part[7], part[8]);
                stat.usr = part[0].parse().unwrap();
                stat.nice = part[1].parse().unwrap();
                break;
            }
        }
        stat.number_of_cpus = get_number_of_cpus().unwrap();
        let cf = db.cf_handle(&self.get_name()).unwrap();
        let value = serde_json::to_vec(&stat).unwrap();
        let key = mario_core::get_current_timestamp().to_string();
        batch.put_cf(cf, key.as_bytes(), &value[..]);
    }
}

fn get_number_of_cpus() -> std::io::Result<u32> {
    let cpuinfo = std::fs::File::open("/proc/cpuinfo")?;
    let buf_reader = BufReader::new(cpuinfo);
    let mut number_of_cpus = 0_u32;
    for line in buf_reader.lines() {
        let line = line?;
        if line.starts_with("processor\t:") {
            number_of_cpus = number_of_cpus + 1;
        }
    }
    Ok(number_of_cpus)
}

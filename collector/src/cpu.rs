use async_trait::async_trait;
use tokio::{fs, io::{AsyncBufReadExt, BufReader}};
use storage::Collector;
pub struct CPUCollector {
    pub name: String,
}

pub struct CPUStat{}

#[async_trait]
impl Collector for CPUCollector {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    async fn collect(&self) {
        let proc_stat = fs::File::open("/proc/stat").await.unwrap();
        let buf_reader = BufReader::new(proc_stat);
        let mut lines = buf_reader.lines();
        while let Some(line) = lines.next_line().await.unwrap() {
            if line.starts_with("cpu ") {
                let part = line[5..].split(" ").collect::<Vec<_>>();
                println!("usr {}, nice {}, sys {}, idle {}, iowait {}, hardirq {}, softirq {}, steal {}, guest {}", part[0], part[1], part[2], part[3], part[4], part[5], part[6], part[7], part[8]);
                break;
            }
        }
        let number_of_cpus = get_number_of_cpus().await.unwrap();
        println!("number of cpu {}", number_of_cpus);
    }
}

async fn get_number_of_cpus() -> std::io::Result<u32> {
    let cpuinfo = tokio::fs::File::open("/proc/cpuinfo").await?;
    let buf_reader = BufReader::new(cpuinfo);
    let mut lines = buf_reader.lines();
    let mut number_of_cpus = 0_u32;
    while let Some(line) = lines.next_line().await? {
        if line.starts_with("processor\t:") {
            number_of_cpus = number_of_cpus + 1;
        }
    }
    Ok(number_of_cpus)
}


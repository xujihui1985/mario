use async_trait::async_trait;
use storage::Collector;
use tokio::{
    fs,
    io::{AsyncBufReadExt, BufReader},
};

pub struct MemCollector {
    pub name: String,
}

#[derive(Default, Debug)]
pub struct MemStat {
    pub totalkb: u64,
    pub freekb: u64,
    pub bufferkb: u64,
    pub cachedkb: u64,
    pub avaliablekb: u64,
}

#[async_trait]
impl Collector for MemCollector {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    async fn collect(&self) {
        let meminfo = fs::File::open("/proc/meminfo").await.unwrap();
        let buf_reader = BufReader::new(meminfo);
        let mut lines = buf_reader.lines();
        let mut stat: MemStat = Default::default();
        while let Some(line) = lines.next_line().await.unwrap() {
            if line.starts_with("MemTotal:") {
                let part =
                    line[9..].trim_start().split(char::is_whitespace).collect::<Vec<_>>()[0];
                stat.totalkb = part.parse().unwrap();
            }
            if line.starts_with("MemFree:") {
                let part =
                    line[8..].trim_start().split(" ").collect::<Vec<_>>()[0];
                stat.freekb = part.parse().unwrap();
            }
            if line.starts_with("Buffers:") {
                let part =
                    line[8..].trim_start().split(" ").collect::<Vec<_>>()[0];
                stat.bufferkb = part.parse().unwrap();
            }
            if line.starts_with("Cached:") {
                let part =
                    line[7..].trim_start().split(" ").collect::<Vec<_>>()[0];
                stat.cachedkb = part.parse().unwrap();
            }
            if line.starts_with("MemAvailable:") {
                let part =
                    line[13..].trim_start().split(" ").collect::<Vec<_>>()[0];
                stat.avaliablekb = part.parse().unwrap();
            }
        }

        println!("collect mem {:?}", stat);
    }
}
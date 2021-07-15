use std::{
    convert::TryInto,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use rocksdb::DB;

fn write_to_db() -> Result<(), Box<dyn std::error::Error>> {
    // let path = DBPath::new("testpath", 10 << 20)?;
    // let mut db_paths = Vec::new();
    // db_paths.push(path);
    let mut opt = rocksdb::Options::default();
    // opt.set_db_paths(&db_paths);
    opt.set_db_write_buffer_size(128 << 20);
    opt.create_missing_column_families(true);
    opt.create_if_missing(true);

    let db = DB::open_cf(&opt, "testpath", &["cpu"])?;
    let cpu_cf = db.cf_handle("cpu").unwrap();
    for i in 0..1000 {
        let container_id = (i - i % 20) / 20;
        let key =
            format!("{}@container{:04}", get_current_timestamp(), container_id);
        println!("put key {}", key);
        db.put_cf(cpu_cf, key.as_bytes(), (i as f64).to_le_bytes())?;
        if i % 50 == 0 {
            std::thread::sleep(Duration::from_secs(5));
        }
    }
    Ok(())
}

fn get_current_timestamp() -> u64 {
    let start = SystemTime::now();
    start.duration_since(UNIX_EPOCH).unwrap().as_secs()
}

fn read_from_db() -> Result<(), Box<dyn std::error::Error>> {
    let mut opt = rocksdb::Options::default();
    // opt.set_db_paths(&db_paths);
    opt.set_db_write_buffer_size(128 << 20);
    opt.create_missing_column_families(true);
    opt.create_if_missing(true);
    let mut table_options = rocksdb::BlockBasedOptions::default();
    table_options.set_bloom_filter(10, true);
    opt.set_block_based_table_factory(&table_options);
    opt.set_prefix_extractor(rocksdb::SliceTransform::create_fixed_prefix(10));

    let db = DB::open_cf(&opt, "testpath", &["cpu"])?;
    let cpu_cf = db.cf_handle("cpu").unwrap();

    let mut readopts = rocksdb::ReadOptions::default();
    readopts.set_iterate_upper_bound("1625897023");
    readopts.set_total_order_seek(false);
    readopts.set_prefix_same_as_start(true);

    let it = db.iterator_cf_opt(
        cpu_cf,
        readopts,
        rocksdb::IteratorMode::From(
            "1625897002".as_bytes(),
            rocksdb::Direction::Forward,
        ),
    );

    let values: Vec<_> = it
        .filter(|(k, _)| {
            let key = std::str::from_utf8(&k).unwrap().to_string();
            key.ends_with("container0037")
        })
        .map(|(k, v)| {
            let key = std::str::from_utf8(&k).unwrap().to_string();
            println!("key is {}", key);
            f64::from_le_bytes((&*v).try_into().unwrap())
        })
        .collect();

    // let values: Vec<_> = it.map(|(k, v)| {
    //          let key = std::str::from_utf8(&k).unwrap().to_string();
    //          println!("key is {}", key);
    //          f64::from_le_bytes((&*v).try_into().unwrap())
    // }).collect();
    // let values: Vec<_> = db
    //     .prefix_iterator_cf(cpu_cf, "1625897002")
    //     .map(|(k, v)| {
    //         let key = std::str::from_utf8(&k).unwrap().to_string();
    //         println!("key is {}", key);
    //         f64::from_le_bytes((&*v).try_into().unwrap())
    //     })
    //     .collect();
    println!("values {:?}", values);

    //    db.raw_iterator_cf_opt(cf_handle, )
    Ok(())
}

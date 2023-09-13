use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

const DB_INDET: i8 = 4;
const DB_PRETTY: bool = true;

const COLLECTION_PATH: &str = "./data/collection.json";

#[derive(Serialize,Deserialize, Debug)]
struct Foo {
    data: String,
    time: f64
}

fn main() {
    create_file(COLLECTION_PATH).unwrap();

    println!("Input data");
    let mut data = String::new();
    std::io::stdin().read_line(&mut data).expect("Input data error");

    // generate data
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let result = Foo { 
        data: data.trim().to_owned(),
        time: now.as_secs_f64()
    };

    // set config
    let mut cfg = jfs::Config::default();
    cfg.single = true;
    cfg.indent = usize::try_from(DB_INDET).unwrap();
    cfg.pretty = DB_PRETTY;

    // connect to DB
    let db = jfs::Store::new_with_cfg(COLLECTION_PATH, cfg).unwrap();
    db.save(&result).expect("Saving data error");

    // print collection
    let collection = db.all::<Foo>().unwrap();
    println!("DB contains:");
    for (s, f) in collection {
        println!("id:{} - {:?}", s, f);
    }

    // let json = serde_json::to_value(collection).unwrap();
}

fn create_file(path: &str) -> Result<(), std::io::Error> {
    create_dir(path)?;
    let path = std::path::Path::new("./home/roger/foo/bar/baz.txt");
    if !path.exists() {
        std::fs::File::create(path).unwrap();
    }

    return Ok(());
}

fn create_dir(path: &str) -> Result<(), std::io::Error>{
    let _path = std::path::Path::new(path);
    if _path.exists() {
        return Ok(());
    }

    let create_dir_result = std::fs::create_dir_all(path);

    match create_dir_result {
        Ok(r) => return Ok(()),
        Err(e) => return Err(e)
    }
}
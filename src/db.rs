use once_cell::sync::Lazy;
use sled::Db;
use std::sync::Mutex;

pub static DB: Lazy<Mutex<Db>> = Lazy::new(|| {
    let db = sled::open("my_db").expect("failed to open database");
    Mutex::new(db)
});



#[test]
fn test() {
    let v = "aleo12j4ptnxyj7vwjgrxxusphndxwejaw3cqmdkkshyqmgmxxg40qq9qnqq7gv."
        .to_string()
        .len();

    println!("{}", v);

    let v2 = "APrivateKey1zkpHRjRfDz33wxRM9GuhT8nh64hVJTC4bfL6P4wD6rgFTNQ"
        .to_owned()
        .len();
    println!("{}", v2);
}

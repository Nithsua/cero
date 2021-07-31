use crate::password::password::{Password, Passwords};
use sled::open;
use sled::IVec;

pub fn add_to_local_store<K: std::convert::AsRef<[u8]>, V: serde::Serialize>(
    key: K,
    value: V,
) -> Result<Option<IVec>, sled::Error> {
    let home_dir = std::env::var("HOME").unwrap();
    // println!("{}", home_dir);
    let local_store = open(format!("{}/.cerostore", home_dir)).expect("Open");
    let serialized_data = bincode::serialize(&value).unwrap();
    local_store.insert(key, serialized_data)
}

pub fn read_from_local_store<K: std::convert::AsRef<[u8]>>(key: K) -> Password {
    let home_dir = std::env::var("HOME").unwrap();
    let local_store = open(format!("{}/.cerostore", home_dir)).expect("Open");
    let serialized_data = local_store.get(key).unwrap().unwrap().to_vec();
    let password: Password = bincode::deserialize(&serialized_data).unwrap();
    password
}

pub fn read_all_the_data() -> Passwords {
    let home_dir = std::env::var("HOME").unwrap();
    let mut passwords = Vec::<Password>::new();
    let local_store = open(format!("{}/.cerostore", home_dir)).unwrap();
    for v in local_store.iter() {
        let (_, v) = v.unwrap();
        let serialized_data = v.to_vec();
        let password: Password = bincode::deserialize(&serialized_data).unwrap();
        passwords.push(password);
    }
    Passwords::new(passwords)
}

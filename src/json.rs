use std::{fs::OpenOptions, io::Read, path::PathBuf, usize};
use super::list_entry::ListEntry;


pub fn load_from_json(path: &PathBuf) -> Vec<ListEntry> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(path)
        .expect("Invalid Path");
    
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);

    serde_json::from_str(&content).expect("Couldn't deserialize json")
}

pub fn append_entry_to_json(path: &PathBuf, entry: ListEntry) -> () {
    let mut entries = load_from_json(path);
    entries.push(entry);

    let file = OpenOptions::new()
        .write(true)
        .open(path)
        .expect("Couldn't open file");

    serde_json::to_writer(file, &entries).expect("Couldn't serialize json");
}

pub fn remove_entry_from_json(path: &PathBuf, index: usize) -> () {
    let mut entries = load_from_json(path);
    entries.remove(index);

    let file = OpenOptions::new()
        .write(true)
        .open(path)
        .expect("Couldn't open file");

    serde_json::to_writer(file, &entries).expect("Couldn't serialize json");
}
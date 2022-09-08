use chrono::Local;

use std::{collections::BTreeMap, error::Error, fs, io::{prelude::*, BufReader, Write}};

pub fn get_date_and_time() -> String {
    Local::now().to_string().replace(":", "_")
}

pub fn store_notes(
    storage: &mut BTreeMap<String, String>,
    idx: &str,
    note: &str,
) -> Result<(), Box<dyn Error>> {
    storage.insert(idx.to_string(), note.to_string());
    Ok(())
}

// FIXME return BTreeMap instead of Vec
pub fn read_file(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = fs::OpenOptions::new().read(true).open(path)?;

    let reader = BufReader::new(file);
    let mut storage: Vec<String> = Vec::new();

    for line in reader.lines() {
        storage.push(line.unwrap());
    }

    Ok(storage)
}

pub fn write_to_file(path: &str, content: &BTreeMap<String, String>) -> Result<(), Box<dyn Error>> {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)?;

    for (key, value) in content {
        writeln!(file, "{}: {}", key, value)?;
    }
    Ok(())
}

use chrono::Local;

use std::{
    collections::BTreeMap,
    error::Error,
    fs,
    io::{prelude::*, BufReader, Write},
};

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

pub fn read_file(
    path: &str,
) -> Result<BTreeMap<String, String>, Box<dyn Error>> {
    let file = fs::OpenOptions::new().read(true).open(path)?;

    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        lines.push(line?);
    }

    let mut storage: BTreeMap<String, String> = BTreeMap::new();

    for line in lines {
        let mut tmp_storage: Vec<&str> = Vec::new();
        tmp_storage = line.split(":").collect();
        storage.insert(
            tmp_storage[0].trim().to_string(),
            tmp_storage[1].trim().to_string(),
        );
        tmp_storage.clear();
    }

    Ok(storage)
}

pub fn write_to_file(
    path: &str,
    content: &BTreeMap<String, String>,
) -> Result<(), Box<dyn Error>> {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)?;

    for (key, value) in content {
        writeln!(file, "{}: {}", key, value)?;
    }
    Ok(())
}

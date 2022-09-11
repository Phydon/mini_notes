// use crate::note::Note;

use chrono::Local;

use std::{
    collections::BTreeMap,
    error::Error,
    fs,
    io::{prelude::*, BufReader, Write},
};

pub fn get_date_and_time() -> String {
    // panics when adding '%.3f' or something similar
    // -> chrono error
    Local::now().format("%a %e %b %Y %T").to_string()
}

pub fn store_notes(
    storage: &mut BTreeMap<String, String>,
    date: &str,
    note: &str,
) -> Result<(), Box<dyn Error>> {
    storage.entry(date.to_string()).or_insert(note.to_string());
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

    // FIXME panics when empty line in file or pattern isn`t as expected
    // 'index out of bounds: the len is 1 but the index is 1', src/lib.rs:44:24
    for line in lines {
        let mut tmp_storage: Vec<&str>;
        tmp_storage = line.split("|").collect();
        storage
            .entry(tmp_storage[0].trim().to_string())
            .or_insert(tmp_storage[1].trim().to_string());
        tmp_storage.clear();
    }

    Ok(storage)
}

pub fn combine_storages<'a>(
    in_storage: &'a mut BTreeMap<String, String>,
    out_storage: &'a mut BTreeMap<String, String>,
) -> Option<BTreeMap<String, String>> {
    // duplicate records (keys) should be impossible
    if out_storage.is_empty() {
        return None;
    } else {
        in_storage.append(out_storage);
    }

    Some(in_storage.clone())
}

// TODO seems to have a problem with longer text
// when writing to txt file
// -> limit it??
pub fn write_to_file(
    path: &str,
    content: &BTreeMap<String, String>,
) -> Result<(), Box<dyn Error>> {
    let mut file =
        fs::OpenOptions::new().write(true).create(true).open(path)?;

    for (key, value) in content {
        writeln!(file, "{} | {}", key, value)?;
    }

    Ok(())
}

// TODO use a database
pub fn database() {
    todo!();
}

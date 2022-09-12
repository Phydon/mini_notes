use crate::note::Note;

use chrono::Local;

use std::{
    error::Error,
    fs,
    io::{prelude::*, BufReader, Write},
    collections::HashSet,
};

pub fn get_date_and_time() -> (String, String) {
    // panics when adding '%.3f' or something similar
    // -> chrono error
    let date = Local::now().format("%a %e %b %Y").to_string();
    let time = Local::now().format("%T").to_string();

    (date, time)
}

pub fn store_note(
    storage: &mut Vec<Note>,
    date: &(String, String),
    txt: &str,
) -> Result<(), Box<dyn Error>> {
    let note: Note = Note::new((date.0.to_string(), date.1.to_string()), txt.to_string())?;
    storage.push(note);

    Ok(())
}

pub fn read_file(path: &str) -> Result<Vec<Note>, Box<dyn Error>> {
    let file = fs::OpenOptions::new().read(true).open(path)?;

    let reader = BufReader::new(file);
    let mut records: Vec<Note> = Vec::new();

    for line in reader.lines() {
        let note = ron::from_str(&line?)?;
        records.push(note);
    }

    Ok(records)
}

fn remove_duplicates(items: &mut Vec<Note>) -> Vec<Note> {
    let mut tmp_set: HashSet<Note> = HashSet::new();
    for item in items {
        tmp_set.insert(item.clone());
    }

    let mut unique_items: Vec<Note> = Vec::new();
    for unique in tmp_set {
        unique_items.push(unique);
    }

    unique_items
}

pub fn combine_storages(
    in_storage: &mut Vec<Note>,
    out_storage: &mut Vec<Note>,
) -> Option<Vec<Note>> {
    let mut unique_storage: Vec<Note>;

    if out_storage.is_empty() {
        return None;
    } else {
        in_storage.append(out_storage);
        unique_storage = remove_duplicates(in_storage);
        unique_storage.sort_by(|a, b| a.date.1.partial_cmp(&b.date.1).unwrap());
    }

    Some(unique_storage)
}

pub fn write_to_file(
    path: &str,
    notes: &Vec<Note>,
) -> Result<(), Box<dyn Error>> {
    let mut file =
        fs::OpenOptions::new().write(true).create(true).open(path)?;

    for note in notes {
        if note.id.to_string() != "00000000-0000-0000-0000-000000000000" {
            writeln!(file, "{}", ron::to_string(&note)?)?;
        }
    }

    Ok(())
}

use crate::note::Note;

use chrono::Local;

use std::{
    error::Error,
    fs,
    io::{prelude::*, BufReader, Write},
};

pub fn get_date_and_time() -> String {
    // panics when adding '%.3f' or something similar
    // -> chrono error
    Local::now().format("%a %e %b %Y %T").to_string()
}

pub fn store_note(
    storage: &mut Vec<Note>,
    date: &str,
    txt: &str,
) -> Result<(), Box<dyn Error>> {
    let note: Note = Note::new(date.to_string(), txt.to_string())?;
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

pub fn combine_storages<'a>(
    in_storage: &'a mut Vec<Note>,
    out_storage: &'a mut Vec<Note>,
) -> Option<Vec<Note>> {
    if out_storage.is_empty() {
        return None;
    } else {
        in_storage.append(out_storage);
    }

    Some(in_storage.clone())
}

pub fn write_to_file(
    path: &str,
    notes: &Vec<Note>,
) -> Result<(), Box<dyn Error>> {
    let mut file =
        fs::OpenOptions::new().write(true).create(true).open(path)?;

    for note in notes {
        writeln!(file, "{}", ron::to_string(&note)?)?;
    }

    Ok(())
}

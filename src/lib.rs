use chrono::Local;

use std::{collections::BTreeMap, error::Error};

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

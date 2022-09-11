use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct Note {
    pub id: Uuid,
    pub date: String,
    pub txt: String,
}

impl Note {
    pub fn new(date: String, txt: String) -> Result<Note, &'static str> {
        if date.is_empty() || txt.is_empty() {
            return Err("Can`t create Note: Not enough arguments");
        }

        let id = Uuid::new_v4();

        Ok(Note { id, date, txt })
    }
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    Default,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
    Deserialize,
    Serialize,
)]
pub struct Note {
    pub id: Uuid,
    pub date: (String, String),
    pub txt: String,
}

impl Note {
    pub fn new(
        date: (String, String),
        txt: String,
    ) -> Result<Note, &'static str> {
        if date.0.is_empty() || date.1.is_empty() || txt.is_empty() {
            return Err("Can`t create Note: Not enough arguments");
        }

        let id = Uuid::new_v4();

        Ok(Note { id, date, txt })
    }
}

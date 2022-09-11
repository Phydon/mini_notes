#[derive(Debug, Clone, Default, PartialEq)]
pub struct Note {
    pub txt: String,
    pub date: String,
}

impl Note {
    pub fn new(date: String, txt: String) -> Note {
        if date.is_empty() || txt.is_empty() {
            panic!("Not enough arguments")
        }

        Note { date, txt }
    }
}

use std::cmp;

pub struct Row {
    contents: String,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            contents: String::from(slice),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.contents.len());
        let start = cmp::min(start, end);
        self.contents
            .get(start..end)
            .unwrap_or_default()
            .to_string()
    }
}

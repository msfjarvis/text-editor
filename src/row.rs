use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    contents: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        let mut row = Self {
            contents: String::from(slice),
            len: 0,
        };
        row.update_len();
        row
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.contents.len());
        let start = cmp::min(start, end);
        let mut result = String::new();
        for grapheme in self.contents[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            if grapheme == "\t" {
                result.push(' ');
            } else {
                result.push_str(grapheme);
            }
        }
        result
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn update_len(&mut self) {
        self.len = self.contents[..].graphemes(true).count();
    }

    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.contents.push(c);
        } else {
            let mut result: String = self.contents[..].graphemes(true).take(at).collect();
            let remainder: String = self.contents[..].graphemes(true).skip(at).collect();
            result.push(c);
            result.push_str(&remainder);
            self.contents = result;
        }
        self.update_len();
    }
}

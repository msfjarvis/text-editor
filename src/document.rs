use crate::Row;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

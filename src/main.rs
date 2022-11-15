mod document;
mod editor;
mod row;
mod terminal;

pub use crate::document::Document;
use crate::editor::Editor;
pub use crate::editor::Position;
pub use crate::row::Row;
pub use crate::terminal::Terminal;

fn main() {
    let mut editor = Editor::default();
    editor.run();
}

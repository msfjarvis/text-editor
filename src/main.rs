mod document;
mod editor;
mod row;
mod terminal;

pub use crate::document::Document;
use crate::editor::Editor;
pub use crate::editor::Position;
pub use crate::row::Row;
pub use crate::terminal::Terminal;
use anyhow::Result;

fn main() -> Result<()> {
    let mut editor = Editor::default();
    editor.run();
    Ok(())
}

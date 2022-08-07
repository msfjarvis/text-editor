mod editor;
mod terminal;

use crate::editor::Editor;
pub use crate::editor::Position;
pub use crate::terminal::Terminal;
use anyhow::Result;

fn main() -> Result<()> {
    let mut editor = Editor::default();
    editor.run();
    Ok(())
}

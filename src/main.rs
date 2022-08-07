mod editor;

use crate::editor::Editor;
use anyhow::Result;

fn main() -> Result<()> {
    let mut editor = Editor::default();
    editor.run();
    Ok(())
}

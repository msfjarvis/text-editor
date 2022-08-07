mod editor;

use crate::editor::Editor;
use anyhow::Result;

fn main() -> Result<()> {
    let editor = Editor::default();
    editor.run()
}

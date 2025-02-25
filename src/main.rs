use std::io;

use editor::Editor;

pub mod editor;
pub mod terminal;

fn main() -> io::Result<()> {
    Editor::default().run()
}

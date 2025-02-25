use std::io;

use editor::Editor;

pub mod editor;

fn main() -> io::Result<()> {
    Editor::default().run()
}

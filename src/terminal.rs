use std::io;

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
}

impl Terminal {
    pub fn defauilt() -> io::Result<Self> {
        let (width, height) = crossterm::terminal::size()?;
        Ok(Self {
            size: Size { width, height },
        })
    }
    pub fn size(&self) -> &Size {
        &self.size
    }
}

use dxgi;
use std::io::ErrorKind::{NotFound, TimedOut, WouldBlock};
use std::{io, ops};

pub struct Capturer {
    inner: dxgi::Capturer,
    width: usize,
    height: usize,
}

impl Capturer {
    pub fn new(display: Display) -> io::Result<Capturer> {
        let width = display.width();
        let height = display.height();
        let inner = dxgi::Capturer::new(&display.0)?;
        Ok(Capturer {
            inner,
            width,
            height,
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn frame<'a>(&'a mut self) -> io::Result<Frame<'a>> {
        const MILLISECONDS_PER_FRAME: u32 = 0;
        match self.inner.frame(MILLISECONDS_PER_FRAME) {
            Ok(frame) => Ok(Frame(frame)),
            Err(ref error) if error.kind() == TimedOut => Err(WouldBlock.into()),
            Err(error) => Err(error),
        }
    }
}

pub struct Frame<'a>(&'a mut [u8]);

impl<'a> ops::Deref for Frame<'a> {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        self.0
    }
}

impl<'a> ops::DerefMut for Frame<'a> {
    fn deref_mut(&mut self) -> &mut [u8] {
        self.0
    }
}

pub struct Display(dxgi::Display);

impl Display {
    pub fn primary() -> io::Result<Display> {
        match dxgi::Displays::new()?.next() {
            Some(inner) => Ok(Display(inner)),
            None => Err(NotFound.into()),
        }
    }

    pub fn all() -> io::Result<Vec<Display>> {
        Ok(dxgi::Displays::new()?.map(Display).collect::<Vec<_>>())
    }

    pub fn width(&self) -> usize {
        self.0.width() as usize
    }

    pub fn height(&self) -> usize {
        self.0.height() as usize
    }
}

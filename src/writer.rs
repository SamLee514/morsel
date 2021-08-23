pub mod tree;

use std::io::{self, Error, ErrorKind, Write};
use termion::{self, clear, cursor, style};

pub struct Writer<W: Write> {
    stdout: W,
    tree: tree::Tree<'static>,
    input_count: u16,
}

impl<W: Write> Writer<W> {
    pub fn new(stdout: W) -> Self {
        Self {
            stdout,
            tree: tree::Tree::new(),
            input_count: 0,
        }
    }

    fn buffered_write(&mut self, content: &str) -> Result<(), Error> {
        write!(self.stdout, "{}", content)?;
        io::stdout().flush()?;
        Ok(())
    }

    pub fn wipe(&mut self) -> Result<(), Error> {
        self.backspace(self.input_count)
    }

    pub fn backspace(&mut self, count: u16) -> Result<(), Error> {
        write!(self.stdout, "{}", cursor::Left(count))?;
        write!(self.stdout, "{}", clear::AfterCursor)?;
        io::stdout().flush()?;
        Ok(())
    }

    pub fn lockout(&mut self) -> Result<(), Error> {
        self.buffered_write(&style::Faint.to_string())?;
        self.buffered_write(&cursor::Hide.to_string())?;
        Ok(())
    }

    pub fn unlock(&mut self) -> Result<(), Error> {
        self.buffered_write(&cursor::Left(1).to_string())?;
        self.buffered_write(&clear::AfterCursor.to_string())?;
        self.buffered_write(&style::NoFaint.to_string())?;
        self.buffered_write(&cursor::Show.to_string())?;
        Ok(())
    }

    pub fn preview_dit(&mut self) -> Result<(), Error> {
        self.buffered_write(".")
    }

    pub fn preview_dah(&mut self) -> Result<(), Error> {
        self.buffered_write(&cursor::Left(1).to_string())?;
        self.buffered_write(&clear::AfterCursor.to_string())?;
        self.buffered_write("_")?;
        Ok(())
    }

    pub fn end_word(&mut self) -> Result<(), Error> {
        self.buffered_write(" ")
    }

    pub fn new_line(&mut self) -> Result<(), Error> {
        self.buffered_write("\n")
    }

    pub fn process_input(&mut self, input: tree::Input) -> Result<(), Error> {
        match self.tree.traverse(input) {
            tree::Output::Value(v) => {
                self.wipe()?;
                self.buffered_write(&v.to_string())?;
                self.input_count = 0;
            }
            tree::Output::Pass => {
                self.input_count += 1;
                match input {
                    tree::Input::Dit => self.buffered_write(".")?,
                    tree::Input::Dah => {
                        // Dahs must always be registered after being first registered as a Dit so it can overwrite
                        self.buffered_write("_")?;
                    }
                    tree::Input::Space => {
                        self.wipe()?;
                        return Err(Error::new(ErrorKind::InvalidInput, "Unexpected space"));
                    }
                };
            }
            tree::Output::Oopsie => {
                self.wipe()?;
                self.input_count = 0;
            }
        }
        Ok(())
    }
}

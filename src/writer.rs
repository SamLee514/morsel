pub mod tree;

use std::fmt::Display;
use std::io::{self, Error, Read, Write};
use std::{
    thread,
    time::{Duration, Instant},
};
use structopt::StructOpt;
use termion::clear;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{self, cursor, event::Key, style};

pub struct Writer<W: Write> {
    stdout: W,
    tree: tree::Tree<'static>,
    input_count: u16,
}

impl<W: Write> Writer<W> {
    pub fn new(stdout: W) -> Self {
        Self {
            stdout: stdout,
            tree: tree::Tree::new(),
            input_count: 0,
        }
    }

    fn buffered_write(&mut self, content: &str) -> Result<(), std::io::Error> {
        write!(self.stdout, "{}", content)?;
        io::stdout().flush()?;
        Ok(())
    }

    fn wipe(&mut self) -> Result<(), std::io::Error> {
        write!(self.stdout, "{}", cursor::Left(self.input_count))?;
        write!(self.stdout, "{}", clear::AfterCursor)?;
        io::stdout().flush()?;
        Ok(())
    }

    pub fn lockout(&mut self) -> Result<(), std::io::Error> {
        self.buffered_write(&style::Faint.to_string())?;
        self.buffered_write(&cursor::Hide.to_string())?;
        Ok(())
    }

    pub fn unlock(&mut self) -> Result<(), std::io::Error> {
        self.buffered_write(&style::NoFaint.to_string())?;
        self.buffered_write(&cursor::Show.to_string())?;
        Ok(())
    }

    pub fn preview_dit(&mut self) -> Result<(), std::io::Error> {
        self.buffered_write(".")
    }

    pub fn preview_dah(&mut self) -> Result<(), std::io::Error> {
        self.buffered_write(&cursor::Left(1).to_string())?;
        self.buffered_write(&clear::AfterCursor.to_string())?;
        self.buffered_write("_")?;
        Ok(())
    }

    pub fn end_word(&mut self) -> Result<(), std::io::Error> {
        self.buffered_write(" ")
    }

    pub fn process_input(&mut self, input: tree::Input) -> Result<(), std::io::Error> {
        match self.tree.traverse(input) {
            tree::Output::Value(v) => {
                self.wipe()?;
                self.buffered_write(&v.to_string())?;
                self.input_count = 0;
            }
            tree::Output::Pass => {
                self.input_count += 1;
                match input {
                    tree::Input::Space => {
                        self.wipe()?;
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "Unexpected space",
                        ));
                    }
                    _ => {}
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

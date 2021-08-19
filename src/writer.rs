pub mod tree;

use std::io::{self, Error, Read, Write};
use std::{
    thread,
    time::{Duration, Instant},
};
use structopt::StructOpt;
use termion::clear;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{self, cursor, event::Key};

pub struct Writer<R: Iterator<Item = Result<Key, std::io::Error>>, W: Write> {
    stdout: W,
    stdin: R,
    tree: tree::Tree<'static>,
    input_count: u16,
}

impl<R: Iterator<Item = Result<Key, std::io::Error>>, W: Write> Writer<R, W> {
    pub fn new(stdin: R, stdout: W) -> Self {
        Self {
            stdin: stdin,
            stdout: stdout,
            tree: tree::Tree::new(),
            input_count: 0,
        }
    }

    fn wipe(&mut self) {
        write!(self.stdout, "{}", cursor::Left(self.input_count)).unwrap();
        write!(self.stdout, "{}", clear::AfterCursor).unwrap();
    }

    pub fn process_input(&mut self, input: tree::Input) -> Result<(), std::io::Error> {
        match self.tree.traverse(input) {
            tree::Output::Value(v) => {
                self.wipe();
                write!(self.stdout, "{}", v)?;
            }
            tree::Output::Pass => {
                self.input_count += 1;
                match input {
                    tree::Input::Dit => write!(self.stdout, ".")?,
                    tree::Input::Dah => write!(self.stdout, "_")?,
                    tree::Input::Space => {
                        self.wipe();
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "Unexpected space",
                        ));
                    }
                };
            }
            tree::Output::Oopsie => {
                self.wipe();
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Oopsie",
                ));
            }
        }
        Ok(())
    }
}

mod tree;

use std::io::{self, Read, Write};
use std::{
    thread,
    time::{Duration, Instant},
};
use structopt::StructOpt;
use termion::clear;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{self, async_stdin, event::Key};

pub struct Writer<R: Iterator<Item = Result<Key, std::io::Error>>, W: Write> {
    stdout: W,
    stdin: R,
    tree: tree::Tree<'static>,
}

impl<R: Iterator<Item = Result<Key, std::io::Error>>, W: Write> Writer<R, W> {
    pub fn new(stdin: R, stdout: W) -> Self {
        Self {
            stdin: stdin,
            stdout: stdout,
            tree: tree::Tree::new(),
        }
    }
}

mod writer;

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

#[derive(Debug, StructOpt)]
#[structopt(name = "morsel", about = "morse code CLI tool")]
struct Opt {
    #[structopt(short, long)]
    help: bool,

    #[structopt(short, long)]
    manual: bool,
}

const DIT_LEN: Duration = Duration::from_millis(500);
const DAH_LEN: Duration = Duration::from_millis(1500);
const LONG_DAH_LEN: Duration = Duration::from_millis(3500);

fn main() {
    let opt = Opt::from_args();
    if opt.help {
        println!("Implementing help screen soon!");
        return;
    }
    if opt.manual {
        println!("Implementing manual soon!");
        return;
    }
    let t
    let writer = writer::Writer::new(async_stdin().keys(), io::stdout().into_raw_mode().unwrap());
    // write!(stdout, "\n").unwrap();

    let mut now;
    let mut was_keydown = false;
    let mut start = Instant::now();
    loop {
        let inp = stdin.next();
        // Print output by the millisecond
        now = start.elapsed();
        if let Some(Ok(c)) = inp {
            match c {
                Key::Char('q') => return,
                _ => {
                    if !was_keydown {
                        was_keydown = true;
                        start = Instant::now();
                    }
                }
            };
        } else {
            // Key up events. Either end a key down or start putting in spaces.
            if was_keydown {
                was_keydown = false;
                // Enter a letter
                if start.elapsed() < DAH_LEN {
                    todo!();
                    // Fire off a dit
                    let result = 
                } else {
                    todo!();
                    // Fire off a dah
                }
                start = Instant::now();
            } else {
                // TODO: possibly direct equality will skip
                if start.elapsed() == LONG_DAH_LEN {
                    todo!();
                    // Fire off a space
                }
                if start.elapsed() == DAH_LEN {
                    todo!();
                    // Fire off a letter check
                }
            }
        }
    }
}

mod writer;

use std::io::{self, Read, Write};
use std::{
    thread,
    time::{Duration, Instant},
};
use structopt::StructOpt;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{self, async_stdin, event::Key};
use termion::{clear, cursor};

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
    write!(io::stdout().into_raw_mode().unwrap(), "\n").unwrap();
    let mut stdin = async_stdin().keys();
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut writer = writer::Writer::new(stdout);

    let mut was_keydown = false;
    let mut start = Instant::now();
    loop {
        let inp = stdin.next();

        if let Some(Ok(c)) = inp {
            match c {
                Key::Char('q') => {
                    return;
                }
                Key::Char('j') => {
                    writer.process_input(writer::tree::Input::Dit);
                }
                Key::Char('k') => {
                    writer.process_input(writer::tree::Input::Dah);
                }
                Key::Char('l') => {
                    writer.process_input(writer::tree::Input::Space);
                }
                _ => {
                    if !was_keydown {
                        was_keydown = true;
                        start = Instant::now();
                    }
                }
            };
        } //else {
          //     // Key up events. Either end a key down or start putting in spaces.
          //     if was_keydown {
          //         was_keydown = false;
          //         // Enter a letter
          //         if start.elapsed() < DAH_LEN {
          //             // Fire off a dit
          //             writer.process_input(writer::tree::Input::Dit);
          //         } else {
          //             // Fire off a dah
          //             writer.process_input(writer::tree::Input::Dah);
          //         }
          //         start = Instant::now();
          //     } else {
          //         // TODO: possibly direct equality will skip
          //         if start.elapsed() == LONG_DAH_LEN {
          //             // Fire off a space
          //             start = Instant::now();
          //         } else if start.elapsed() == DAH_LEN {
          //             // Fire off a letter check. If the letter failed, reset the timer so we don't fire a space too early.
          //             match writer.process_input(writer::tree::Input::Space) {
          //                 Ok(_) => {}
          //                 Err(_) => start = Instant::now(),
          //             }
          //         }
          //     }
          // }
    }
}

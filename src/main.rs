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

fn main() -> Result<(), std::io::Error> {
    let opt = Opt::from_args();
    if opt.help {
        println!("Implementing help screen soon!");
        return Ok(());
    }
    if opt.manual {
        println!("Implementing manual soon!");
        return Ok(());
    }
    write!(io::stdout().into_raw_mode().unwrap(), "\n").unwrap();
    let mut stdin = async_stdin().keys();
    let stdout = io::stdout().into_raw_mode().unwrap();
    let mut writer = writer::Writer::new(stdout);

    let mut start = Instant::now();
    /**
     * (Input) Keydown -> Pause => Dit
     * (Input) Keydown -> Hold => Dah
     * Input -> Short pause before next keydown => Pass
     * Input -> Medium pause before next keydown => Letter check
     * Input -> Long pause before next keydown => Letter check and end word
     * Continued holding => lockout
     * Continued pause => lockout
     */
    let mut do_nothing_until_keydown = true;
    let mut check_processed = false;
    loop {
        if let Some(Ok(c)) = stdin.next() {
            check_processed &= false;
            start = Instant::now();
            match c {
                Key::Char('q') => {
                    writer.unlock()?;
                    return Ok(());
                }
                _ => {
                    do_nothing_until_keydown = false;
                    let mut holding = false;
                    let mut input = writer::tree::Input::Dit;
                    writer.lockout()?;
                    writer.preview_dit()?;
                    loop {
                        // writer.end_word()?;
                        if start.elapsed() >= DIT_LEN {
                            writer.process_input(input)?;
                            break;
                        } else if let Some(Ok(c1)) = stdin.next() {
                            match c1 {
                                Key::Char('q') => {
                                    writer.unlock()?;
                                    return Ok(());
                                }
                                _ => {
                                    if !holding {
                                        holding = true;
                                        input = writer::tree::Input::Dah;
                                        writer.preview_dah()?;
                                    }
                                    start = Instant::now();
                                }
                            }
                        }
                    }
                    writer.unlock()?;
                }
            };
        } else if !do_nothing_until_keydown {
            let elapsed = start.elapsed();
            if elapsed >= DAH_LEN && !check_processed {
                match writer.process_input(writer::tree::Input::Space) {
                    Ok(_) => check_processed = true,
                    Err(_) => {
                        do_nothing_until_keydown = true;
                        start = Instant::now();
                    }
                }
            } else if elapsed >= LONG_DAH_LEN {
                do_nothing_until_keydown = true;
                writer.end_word()?;
            }
        }
    }
}

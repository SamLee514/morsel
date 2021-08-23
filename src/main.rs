mod help;
mod writer;

use std::io::{self, Stdout, Write};
use std::time::{Duration, Instant};
use structopt::StructOpt;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{self, async_stdin, event::Key, input::Keys, AsyncReader};

#[derive(Debug, StructOpt)]
#[structopt(name = "morsel", about = "morse code CLI tool")]
struct Opt {
    #[structopt(short, long)]
    help: bool,

    #[structopt(short, long)]
    manual: bool,

    #[structopt(short, long, default_value = "500")]
    dit_length: u64,
}

pub fn main_loop(
    mut stdin: Keys<AsyncReader>,
    mut writer: writer::Writer<RawTerminal<Stdout>>,
    dit_length: u64,
) -> Result<(), std::io::Error> {
    let dit_len: Duration = Duration::from_millis(dit_length);
    let dah_len: Duration = Duration::from_millis(3 * dit_length);
    let long_dah_len: Duration = Duration::from_millis(7 * dit_length);

    let mut do_nothing_until_keydown = true;
    let mut check_processed = false;

    let mut start = Instant::now();
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
                        if start.elapsed() >= dit_len {
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
                    writer.process_input(input)?;
                }
            };
        } else if !do_nothing_until_keydown {
            let elapsed = start.elapsed();
            if elapsed >= dah_len && !check_processed {
                match writer.process_input(writer::tree::Input::Space) {
                    Ok(_) => check_processed = true,
                    Err(_) => {
                        do_nothing_until_keydown = true;
                        start = Instant::now();
                    }
                }
            } else if elapsed >= long_dah_len {
                do_nothing_until_keydown = true;
                writer.end_word()?;
            }
        }
    }
}

pub fn manual_loop(
    mut stdin: Keys<AsyncReader>,
    mut writer: writer::Writer<RawTerminal<Stdout>>,
) -> Result<(), std::io::Error> {
    println!("manual loop");
    let mut word_checked = false;
    loop {
        if let Some(Ok(c)) = stdin.next() {
            match c {
                Key::Char('q') => {
                    return Ok(());
                }
                Key::Char('.') => {
                    writer.process_input(writer::tree::Input::Dit)?;
                    word_checked = false;
                }
                Key::Char('_') => {
                    writer.process_input(writer::tree::Input::Dah)?;
                    word_checked = false;
                }
                Key::Char(' ') => match word_checked {
                    true => {
                        writer.end_word()?;
                    }
                    false => {
                        writer.process_input(writer::tree::Input::Space)?;
                        word_checked = true;
                    }
                },
                Key::Backspace => {
                    writer.backspace(1)?;
                }
                _ => {}
            }
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let opt = Opt::from_args();
    assert!(
        !(opt.manual && opt.help),
        "Cannot do manual mode and help simultaneously >:("
    );
    write!(io::stdout().into_raw_mode().unwrap(), "\n").unwrap();
    if opt.help {
        help::show()
    } else {
        let stdin = async_stdin().keys();
        let writer = writer::Writer::new(io::stdout().into_raw_mode().unwrap());
        if opt.manual {
            manual_loop(stdin, writer)
        } else {
            main_loop(stdin, writer, opt.dit_length)
        }
    }
}

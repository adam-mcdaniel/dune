extern crate clap;

use std::fs;
use std::io;
use std::path::Path;
use self::clap::ArgMatches;
use super::CommentStyle;

#[derive(Debug)]
pub enum Input {
    Standard(io::Stdin),
    File(fs::File)
}

impl Input {
    fn stdin() -> Input {
        Input::Standard(io::stdin())
    }
    fn file<P: AsRef<Path>>(path: P) -> io::Result<Input> {
        Ok(Input::File(try!(fs::File::open(path))))
    }
    fn from_arg<P: AsRef<Path>>(arg: Option<P>) -> io::Result<Input> {
        Ok(match arg {
            None       => Input::stdin(),
            Some(path) => try!(Input::file(path))
        })
    }
}

impl io::Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            Input::Standard(ref mut s) => s.read(buf),
            Input::File(ref mut f)     => f.read(buf),
        }
    }
}

#[derive(Debug)]
pub enum Output {
    Standard(io::Stdout),
    File(fs::File)
}

impl Output {
    fn stdout() -> Output {
        Output::Standard(io::stdout())
    }
    fn file<P: AsRef<Path>>(path: P) -> io::Result<Output> {
        Ok(Output::File(try!(fs::File::create(path))))
    }
    fn from_arg<P: AsRef<Path>>(arg: Option<P>) -> io::Result<Output> {
        Ok(match arg {
            None       => Output::stdout(),
            Some(path) => try!(Output::file(path))
        })
    }
}

impl io::Write for Output {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match *self {
            Output::Standard(ref mut s) => s.write(buf),
            Output::File(ref mut f)     => f.write(buf),
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        match *self {
            Output::Standard(ref mut s) => s.flush(),
            Output::File(ref mut f)     => f.flush(),
        }
    }
}

pub struct Config {
    pub input: Input,
    pub output: Output,
    pub style: CommentStyle,
    pub remove_blanks: bool
}

impl Config {
    pub fn from_matches(matches: &ArgMatches) -> io::Result<Self> {
        let style_arg = (matches.is_present("c-style"),
                        matches.is_present("xml-style"),
                        matches.is_present("shell-style"));
        let comment_style = match style_arg {
            (true, _, _) => CommentStyle::C,
            (_, true, _) => CommentStyle::XML,
            (_, _, true) => CommentStyle::Shell,
            _ => CommentStyle::Shell
        };
        Ok(Config {
            input: Input::from_arg(matches.value_of("INPUT"))?,
            output: Output::from_arg(matches.value_of("output"))?,
            style: comment_style,
            remove_blanks: !matches.is_present("no-remove-blank-lines")
        })   
    }
}

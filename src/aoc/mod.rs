#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

pub mod algorithm;
pub mod math;
pub mod point2d;

use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use std::time::Instant;
use std::{io, result};

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    ParseFailed,
    FromProblem(String),
}
impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IO(e)
    }
}

pub type Result<T> = result::Result<T, Error>;

#[allow(clippy::missing_errors_doc)]
pub fn file(filename: &str) -> Result<BufReader<File>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file))
}

#[allow(clippy::missing_errors_doc)]
pub fn parse<T: FromStr>(line: io::Result<String>) -> Result<T> {
    #[allow(clippy::map_err_ignore)]
    line?.parse::<T>().map_err(|_| Error::ParseFailed)
}

#[must_use]
pub fn parse_unwrap<T: FromStr>(line: io::Result<String>) -> T {
    parse::<T>(line).unwrap()
}

pub struct CodeTimer(Instant, Instant, bool);
impl CodeTimer {
    #[must_use]
    pub fn new() -> Self {
        Self(Instant::now(), Instant::now(), false)
    }

    pub fn split(&mut self, what: &str) {
        println!(
            "{} took {:?} ({:?} total)",
            what,
            self.0.elapsed(),
            self.1.elapsed()
        );
        self.0 = Instant::now();
    }

    pub fn stop(&mut self, what: &str) {
        println!(
            "{} took {:?} ({:?} total)",
            what,
            self.0.elapsed(),
            self.1.elapsed()
        );
        self.2 = true;
    }
}
impl Default for CodeTimer {
    fn default() -> Self {
        CodeTimer::new()
    }
}
impl Drop for CodeTimer {
    fn drop(&mut self) {
        if !self.2 {
            println!("Took {:?} ({:?} total)", self.0.elapsed(), self.1.elapsed());
        }
    }
}

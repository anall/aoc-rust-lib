#![warn( clippy::all, clippy::pedantic )]

pub mod algorithm;

use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use std::{io, result};

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    ParseFailed,
    FromProblem(String)
}
impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IO(e)
    }
}

pub type Result<T> = result::Result<T,Error>;

#[allow( clippy::missing_errors_doc )]
pub fn file(filename : &str) -> Result<BufReader<File>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file))
}

#[allow( clippy::missing_errors_doc )]
pub fn parse<T: FromStr>(line : io::Result<String>) -> Result<T>{
    #[allow( clippy::map_err_ignore )]
    line?.parse::<T>().map_err(|_| Error::ParseFailed)
}

#[must_use]
pub fn parse_unwrap<T: FromStr>(line : io::Result<String>) -> T {
    parse::<T>(line).unwrap()
}

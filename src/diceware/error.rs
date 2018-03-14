use std::error;
use std::fmt;
use std::io;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    WordList(WordListError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IO(ref err) => err.fmt(f),
            Error::WordList(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IO(ref err) => err.description(),
            Error::WordList(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IO(ref err) => Some(err),
            Error::WordList(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<WordListError> for Error {
    fn from(err: WordListError) -> Error {
        Error::WordList(err)
    }
}

#[derive(Debug)]
pub enum WordListError {
    InvalidLength(usize),
}

impl fmt::Display for WordListError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WordListError::InvalidLength(ref length) => {
                write!(f, "Word list: invalid length ({})", length)
            }
        }
    }
}

impl error::Error for WordListError {
    fn description(&self) -> &str {
        match *self {
            WordListError::InvalidLength(_) => "Invalid word list length",
        }
    }
}

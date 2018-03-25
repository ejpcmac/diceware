use std::error;
use std::fmt;
use std::io;
use std::result;

/// Short hand for the
/// [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) type.
pub type Result<T> = result::Result<T, Error>;

/// Diceware errors.
#[derive(Debug)]
pub enum Error {
    /// IO errors, typically encountered when trying to read a word list from a
    /// file.
    IO(io::Error),

    /// Word list errors, encountered when the word list is invalid.
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

/// Word list errors.
#[derive(Debug)]
pub enum WordListError {
    /// Error for when the word list is not 7776-word long.
    InvalidLength(usize),

    /// Error for when the word list contains duplicates.
    DuplicateWord(String),
}

impl fmt::Display for WordListError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WordListError::InvalidLength(ref length) => {
                write!(f, "Word list: invalid length ({})", length)
            }

            WordListError::DuplicateWord(ref word) => {
                write!(f, "Word list: {}: duplicate word", word)
            }
        }
    }
}

impl error::Error for WordListError {
    fn description(&self) -> &str {
        match *self {
            WordListError::InvalidLength(_) => "Invalid word list length",
            WordListError::DuplicateWord(_) => "Duplicate word in the list",
        }
    }
}

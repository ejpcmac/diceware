// diceware - A Diceware passphrase generator.
// Copyright (C) 2018 Jean-Philippe Cugnet <jean-philippe@cugnet.eu>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

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

    /// Error for when the number of words to generate is 0.
    NoWords,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IO(err) => err.fmt(f),
            Error::WordList(err) => err.fmt(f),
            Error::NoWords => write!(f, "No words to generate"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::IO(err) => err.description(),
            Error::WordList(err) => err.description(),
            Error::NoWords => "No words to generate",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            Error::IO(err) => Some(err),
            Error::WordList(err) => Some(err),
            Error::NoWords => None,
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
        match self {
            WordListError::InvalidLength(length) => {
                write!(f, "Word list: invalid length ({})", length)
            }

            WordListError::DuplicateWord(word) => {
                write!(f, "Word list: {}: duplicate word", word)
            }
        }
    }
}

impl error::Error for WordListError {
    fn description(&self) -> &str {
        match self {
            WordListError::InvalidLength(_) => "Invalid word list length",
            WordListError::DuplicateWord(_) => "Duplicate word in the list",
        }
    }
}

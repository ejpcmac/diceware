use std::error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use rand;
use rand::Rng;

pub struct Config<'a> {
    word_list: WordList<'a>,
    words: u32,
    with_special_char: bool,
}

impl<'a> Config<'a> {
    pub fn with_filename(
        filename: &'a str,
        words: u32,
        with_special_char: bool,
    ) -> Config<'a> {
        Config {
            word_list: WordList::File(filename),
            words,
            with_special_char,
        }
    }
}

enum WordList<'a> {
    File(&'a str),
}

impl<'a> WordList<'a> {
    fn get(&self) -> Result<Vec<String>> {
        match *self {
            WordList::File(filename) => get_wordlist(filename),
        }
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    WordList(WordListError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::WordList(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => err.description(),
            Error::WordList(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::WordList(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<WordListError> for Error {
    fn from(err: WordListError) -> Error {
        Error::WordList(err)
    }
}

#[derive(Debug)]
pub enum WordListError {
    InvalidLength,
}

impl fmt::Display for WordListError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Word list: invalid length")
    }
}

impl error::Error for WordListError {
    fn description(&self) -> &str {
        match *self {
            WordListError::InvalidLength => "Word list: invalid length",
        }
    }
}

pub fn make_passphrase(config: Config) -> Result<String> {
    let word_list = config.word_list.get()?;
    let mut passphrase = String::new();

    for _ in 0..config.words {
        let word = rand::thread_rng().choose(&word_list).unwrap();
        passphrase.push_str(word);
        passphrase.push_str(" ");
    }

    // Pop the trailing space.
    passphrase.pop();

    if config.with_special_char {
        let chars: Vec<char> =
            "~!#$%^&*()-=+[]\\{}:;\"'<>?/0123456789".chars().collect();

        let c = rand::thread_rng().choose(&chars).unwrap();

        // TODO: Avoid len().
        let position = rand::thread_rng().gen_range(0, passphrase.len());
        passphrase.insert(position, *c);
    }

    Ok(passphrase)
}

fn get_wordlist<P: AsRef<Path>>(filename: P) -> Result<Vec<String>> {
    let mut content = String::new();
    File::open(filename)?.read_to_string(&mut content)?;

    if content.lines().count() != 7776 {
        return Err(From::from(WordListError::InvalidLength));
    }

    let word_list = content.lines().map(String::from).collect();

    Ok(word_list)
}

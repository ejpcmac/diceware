use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use rand;
use rand::Rng;

use self::WordListError::InvalidLength;
pub use self::error::{Error, Result, WordListError};

mod error;

pub struct Config<'a> {
    word_list: WordList<'a>,
    words: usize,
    with_special_char: bool,
}

impl<'a> Config<'a> {
    pub fn with_filename(
        filename: &'a str,
        words: usize,
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

fn get_wordlist<P>(filename: P) -> Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let mut content = String::new();
    File::open(filename)?.read_to_string(&mut content)?;

    let length = content.lines().count();
    if length != 7776 {
        return Err(Error::WordList(InvalidLength(length)));
    }

    let word_list = content.lines().map(String::from).collect();
    Ok(word_list)
}

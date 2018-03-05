use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::error::Error;

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
    fn get(&self) -> Result<Vec<String>, Box<Error>> {
        match self {
            &WordList::File(filename) => get_wordlist(filename),
        }
    }
}

pub fn make_passphrase(config: Config) -> Result<String, Box<Error>> {
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

fn get_wordlist<P>(filename: P) -> Result<Vec<String>, Box<Error>>
where
    P: AsRef<Path>,
{
    let mut content = String::new();
    File::open(filename)?.read_to_string(&mut content)?;
    let lines = content.lines();

    // TODO: Handle lines.length() != 7776.
    // if lines.count() != 7776 {
    //     return Err();
    // }

    let word_list = lines.map(String::from).collect();

    Ok(word_list)
}

//! A Diceware passphrase generator.
//!
//! # About
//!
//! [Diceware](http://world.std.com/~reinhold/diceware.html) is a method by
//! Arnold G. Reinhold for generating passphrases from a dice and a word list.
//!
//! Although you should not trust your computer’s pseudo-random number generator
//! for generating strong passphrases, it is sometimes convenient and acceptable
//! to generate passphrases that are easy to remember, yet less secure than a
//! true Diceware passphrase.
//!
//! # Features
//!
//! This Diceware implementation enables to generate passphrases from a Diceware
//! word list, with an optional special character insered at any position. This
//! differs from the dice version, where the special character can be inserted
//! only in the six first characters of the six first words.
//!
//! This implementation embeds two word lists:
//!
//!   * the original Diceware list,
//!   * the French word list from
//!     [Matthieu Weber](http://weber.fi.eu.org/index.shtml.en#projects), with
//!     `Église` changed to `Eglise` to avoid encoding issues.
//!
//! In addition to these lists, you can use any other list from a text file
//! featuring a word by line. A word list **must** contain exactly 7776 *unique*
//! words.
//!
//! Before each passphrase generation, the chosen word list is checked so that
//! you do not need to trust its creator. This is also the case for embedded
//! lists, so you do not have to trust me either: just read the source code and
//! acknowledge by yourself you can use trustless word lists.
//!
//! # Usage
//!
//! ## As a binary
//!
//! The simplest way to use the `diceware` binary is to just pass the number of
//! desired words as an argument:
//!
//!     $ diceware 8
//!     save andrew liar grater keys chad poetry stole
//!
//! In this case, the embedded original Diceware list is used and no special
//! character is added.
//!
//! To add a special character, add the `-s` switch:
//!
//!     $ diceware -s 8
//!     clerk ion ruddy aid gauss wino listen fl>o
//!
//! To use the embedded French word list, use `--fr`:
//!
//!     $ diceware --fr 8
//!     jarret papa cv asti brin coron rente don
//!
//! You can also use any external word list:
//!
//!     $ diceware 8 -f word_list.txt
//!     yah omaha aiken wood noble shoot devil filch
//!
//! ## As a library
//!
//! Add this crate as a dependency to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! diceware = { git = "https://github.com/ejpcmac/diceware.git", tag = "v1.0.0" }
//! ```
//!
//! Then, add this to your crate root:
//!
//! ```
//! extern crate diceware;
//! ```
//!
//! ### Example
//!
//! ```
//! use diceware::{Config, EmbeddedList, Error};
//!
//! // First, generate a config. You can generate one with an embedded list,
//! // here the English one with 8 words and without a special character:
//! let config = Config::with_embedded(EmbeddedList::EN, 8, false);
//!
//! // Alternatively, you can generate a config using an external word list. For
//! // instance, to generate 6 words from the file `list.txt` with an additional
//! // special character:
//! let filename = "list.txt";
//! let config = Config::with_filename(filename, 8, true);
//!
//! // Then, try to generate the passphrase:
//! match diceware::make_passphrase(config) {
//!     // The happy path: you get your passphrase.
//!     Ok(passphrase) => println!("{}", passphrase),
//!
//!     // Some errors can occur:
//!     Err(err) => {
//!         match err {
//!             // IO errors can occur when using an external word list.
//!             Error::IO(ref e) => eprintln!("Error: {}: {}", filename, e),
//!
//!             // Word list errors can occur if the word list is invalid, i.e.
//!             // its length is different than 7776 words or it contains
//!             // duplicates.
//!             Error::WordList(ref e) => eprintln!("Error: {}", e),
//!         }
//!     }
//! };
//! ```

extern crate rand;

use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use rand::Rng;
use rand::os::OsRng;

use self::WordListError::*;
pub use self::error::*;

mod embedded;
mod error;

/// The list of embedded word lists.
pub enum EmbeddedList {
    /// The original English Diceware word list.
    EN,

    /// [Matthieu Weber](http://weber.fi.eu.org/index.shtml.en#projects)’s
    /// French word list.
    ///
    /// To avoid encoding or accessibility problems, `Église` has been replaced
    /// by `Eglise` in the list.
    FR,
}

/// Configuration for the passphrase generator.
///
/// To create a configuration, you must use one of the constructors:
///
/// * [`Config::with_filename`](#method.with_filename)
/// * [`Config::with_embedded`](#method.with_embedded)
pub struct Config<'a> {
    word_list: WordList<'a>,
    words: usize,
    with_special_char: bool,
}

impl<'a> Config<'a> {
    /// Creates a configuration using an external word list.
    ///
    /// # Example
    ///
    /// ```
    /// // Create a configuration to generate 8 words with a special char,
    /// // using the word list in words.txt:
    /// let config = Config::with_filename("words.txt", 8, true);
    /// ```
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

    /// Creates a configuration using an embedded word list.
    ///
    /// # Example
    ///
    /// ```
    /// // Create a configuration to generate 6 words without a special char,
    /// // using the embedded French word list:
    /// let config = Config::with_embedded(EmbeddedList::FR, 6, false);
    /// ```
    pub fn with_embedded(
        list: EmbeddedList,
        words: usize,
        with_special_char: bool,
    ) -> Config<'a> {
        Config {
            word_list: WordList::Embedded(list),
            words,
            with_special_char,
        }
    }
}

/// A word list.
enum WordList<'a> {
    File(&'a str),
    Embedded(EmbeddedList),
}

impl<'a> WordList<'a> {
    /// Gets the word list a a vector of strings.
    fn get(&self) -> Result<Vec<String>> {
        let word_list = match *self {
            WordList::File(filename) => get_wordlist(filename)?,
            WordList::Embedded(ref list) => get_embedded_list(list),
        };

        // Add a block to limit the scope of the &word_list borrow.
        {
            // Check the list for duplicates.
            let mut hash_list = HashSet::<&str>::new();
            for word in &word_list {
                if !hash_list.insert(word) {
                    return Err(Error::WordList(DuplicateWord(word.clone())));
                }
            }
        }

        Ok(word_list)
    }
}

/// Makes a passphrase given a [`config`](./struct.Config.html).
///
/// # Example
///
/// ```
/// // Make an 8-word passphrase from the embedded English list.
/// let config = Config::with_embedded(EmbeddedList::EN, 8, false);
/// let passphrase = make_passphrase(config).unwrap();
/// ```
///
/// If the list can generate an error, like when you use an external list or
/// if you don’t trust the embedded lists, you can match them:
///
/// ```
/// match make_passphrase(config) {
///     Ok(passphrase) => println!("{}", passphrase),
///
///     Err(err) => {
///         match err {
///             // IO errors can occur when using an external word list.
///             Error::IO(ref e) => eprintln!("Error: {}: {}", filename, e),
///
///             // Word list errors can occur if the word list is invalid, i.e.
///             // its length is different than 7776 words or it contains
///             // duplicates.
///             Error::WordList(ref e) => eprintln!("Error: {}", e),
///         }
///     }
/// };
/// ```
pub fn make_passphrase(config: Config) -> Result<String> {
    let mut rng = OsRng::new().unwrap();

    let word_list = config.word_list.get()?;
    let mut passphrase = (0..config.words).fold(String::new(), |s, _| {
        s + rng.choose(&word_list).unwrap() + " "
    });

    // Pop the trailing space.
    passphrase.pop();

    if config.with_special_char {
        let chars: Vec<char> = "~!#$%^&*()-=+[]\\{}:;\"'<>?/0123456789"
            .chars()
            .collect();

        let c = rng.choose(&chars).unwrap();

        // TODO: Avoid len().
        let position = rng.gen_range(0, passphrase.len());
        passphrase.insert(position, *c);
    }

    Ok(passphrase)
}

/// Gets the word list from a file.
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

/// Gets an embedded word list.
fn get_embedded_list(list: &EmbeddedList) -> Vec<String> {
    let word_list = match *list {
        EmbeddedList::EN => &embedded::EN,
        EmbeddedList::FR => &embedded::FR,
    };

    word_list
        .iter()
        .map(|&w| String::from(w))
        .collect()
}

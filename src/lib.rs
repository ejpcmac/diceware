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
//! word list, with an optional special character inserted at any position in
//! any word. This differs from the dice version, where the special character
//! can be inserted only in the six first characters of the six first words.
//!
//! This implementation embeds two word lists:
//!
//!   * the original Diceware list,
//!   * the French word list from
//!     [Matthieu Weber](http://weber.fi.eu.org/index.shtml.en#projects), with
//!     `Église` changed to `Eglise` to avoid encoding and keyboard
//!     accessibility issues.
//!
//! In addition to these lists, you can use any other list from a text file
//! featuring a word by line. A word list **must** contain exactly 7776 unique
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
//! ```sh
//! $ diceware 8
//! save andrew liar grater keys chad poetry stole
//! ```
//!
//! In this case, the embedded original Diceware list is used and no special
//! character is added.
//!
//! To add a special character, use the `-s` switch:
//!
//! ```sh
//! $ diceware -s 8
//! clerk ion ruddy aid gauss wino listen fl>o
//! ```
//!
//! To use the embedded French word list, use `--fr`:
//!
//! ```sh
//! $ diceware --fr 8
//! jarret papa cv asti brin coron rente don
//! ```
//!
//! You can also use any external word list:
//!
//! ```sh
//! $ diceware 8 -f word_list.txt
//! yah omaha aiken wood noble shoot devil filch
//! ```
//!
//! ## As a library
//!
//! Add this crate as a dependency to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! diceware = { git = "https://github.com/ejpcmac/diceware.git", tag = "v1.0.1" }
//! ```
//!
//! Then, add this to your crate root:
//!
//! ```rust
//! extern crate diceware;
//! ```
//!
//! ### Example
//!
//! ```rust
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
//!             Error::IO(e) => eprintln!("Error: {}: {}", filename, e),
//!
//!             // Word list errors can occur if the word list is invalid, i.e.
//!             // its length is different than 7776 words or it contains
//!             // duplicates.
//!             Error::WordList(e) => eprintln!("Error: {}", e),
//!
//!             // No words errors can occur if the number of words to generate
//!             // is 0.
//!             Error::NoWords => eprintln!("Error: {}", err),
//!         }
//!     }
//! };
//! ```

#![warn(rust_2018_idioms)]
#![warn(clippy::redundant_pub_crate)]
#![warn(clippy::use_self)]
#![deny(missing_docs)]
#![deny(unused_must_use)]
#![forbid(unsafe_code)]

mod embedded;
mod error;

pub use self::error::*;

use std::{collections::HashSet, fs, path::Path};

use rand::{prelude::*, rngs::OsRng};
use unicode_segmentation::UnicodeSegmentation;

use self::error::WordListError::*;

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

/// A word list.
enum WordList<'a> {
    File(&'a str),
    Embedded(EmbeddedList),
}

/// The list of embedded word lists.
#[derive(Clone, Debug)]
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

impl<'a> Config<'a> {
    /// Creates a configuration using an external word list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use diceware::Config;
    ///
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
    /// ```rust
    /// use diceware::{Config, EmbeddedList};
    ///
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

impl<'a> WordList<'a> {
    /// Gets the word list a a vector of strings.
    fn get(&self) -> Result<Vec<String>> {
        let word_list = match self {
            WordList::File(filename) => get_wordlist(filename)?,
            WordList::Embedded(list) => get_embedded_list(list),
        };

        // This block limits the scope of the &word_list borrow.
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
/// ```rust
/// use diceware::{Config, EmbeddedList};
///
/// // Make an 8-word passphrase from the embedded English list.
/// let config = Config::with_embedded(EmbeddedList::EN, 8, false);
/// let passphrase = diceware::make_passphrase(config).unwrap();
/// ```
///
/// If the list can generate an error, like when you use an external list or
/// if you don’t trust the embedded lists, you can match them:
///
/// ```rust
/// use diceware::{Config, EmbeddedList, Error};
///
/// let filename = "words.txt";
/// let config = Config::with_filename(filename, 8, false);
/// match diceware::make_passphrase(config) {
///     Ok(passphrase) => println!("{}", passphrase),
///
///     Err(err) => {
///         match err {
///             // IO errors can occur when using an external word list.
///             Error::IO(e) => eprintln!("Error: {}: {}", filename, e),
///
///             // Word list errors can occur if the word list is invalid, i.e.
///             // its length is different than 7776 words or it contains
///             // duplicates.
///             Error::WordList(e) => eprintln!("Error: {}", e),
///
///             // No words errors can occur if the number of words to generate
///             // is 0.
///             Error::NoWords => eprintln!("Error: {}", err),
///         }
///     }
/// };
/// ```
pub fn make_passphrase(config: Config<'_>) -> Result<String> {
    if config.words < 1 {
        return Err(Error::NoWords);
    }

    let mut rng = OsRng;

    // We need to declare this mutable string before `word_list` if we want to
    // use it to replace a word with its version containing a special character.
    let mut word = String::new();

    let word_list = config.word_list.get()?;
    let mut words: Vec<&str> = (0..config.words)
        .map(|_| word_list.choose(&mut rng).unwrap())
        .map(AsRef::as_ref)
        .collect();

    if config.with_special_char {
        let chars: Vec<char> =
            "~!#$%^&*()-=+[]\\{}:;\"'<>?/0123456789".chars().collect();

        let c = chars.choose(&mut rng).unwrap();

        let word_idx = rng.gen_range(0..words.len());
        word.push_str(words[word_idx]);

        let indices: Vec<usize> =
            word.grapheme_indices(true).map(|(i, _)| i).collect();

        let idx = indices.choose(&mut rng).unwrap();

        word.insert(*idx, *c);
        words[word_idx] = &word;
    }

    let passphrase = words.join(" ");

    Ok(passphrase)
}

/// Gets the word list from a file.
fn get_wordlist(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    let content = fs::read_to_string(filename)?;

    let length = content.lines().count();
    if length != 7776 {
        return Err(Error::WordList(InvalidLength(length)));
    }

    let word_list = content.lines().map(String::from).collect();
    Ok(word_list)
}

/// Gets an embedded word list.
fn get_embedded_list(list: &EmbeddedList) -> Vec<String> {
    embedded_list(list)
        .iter()
        .map(|&w| String::from(w))
        .collect()
}

/// Gets the corresponding embedded word list.
fn embedded_list(list: &EmbeddedList) -> &[&str; 7776] {
    match list {
        EmbeddedList::EN => &embedded::EN,
        EmbeddedList::FR => &embedded::FR,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    /// Arbitrary embedded word list generator.
    fn arb_list() -> BoxedStrategy<EmbeddedList> {
        prop_oneof![Just(EmbeddedList::EN), Just(EmbeddedList::FR)].boxed()
    }

    #[test]
    fn returns_an_error_if_number_of_words_is_zero() {
        let config = Config::with_embedded(EmbeddedList::FR, 0, false);
        let result = make_passphrase(config);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "No words to generate");
    }

    proptest! {
        #[test]
        fn makes_a_passphrase(ref list in arb_list(), n in 1..50usize) {
            let word_list = embedded_list(list);

            let config = Config::with_embedded(list.clone(), n, false);
            let result = make_passphrase(config);

            prop_assert!(result.is_ok());
            prop_assert!(
                result
                    .unwrap()
                    .split_whitespace()
                    .all(|w| word_list.contains(&w))
            );
        }
    }

    proptest! {
        #[test]
        fn makes_a_passphrase_with_special_char(
            ref list in arb_list(),
            n in 1..50usize
        ) {
            let word_list = embedded_list(list);

            let config = Config::with_embedded(list.clone(), n, true);
            let result = make_passphrase(config);

            prop_assert!(result.is_ok());

            let passphrase = result.unwrap();
            let not_in_wordlist: Vec<&str> = passphrase
                .split_whitespace()
                .filter(|w| !word_list.contains(w))
                .collect();

            prop_assert_eq!(not_in_wordlist.len(), 1);

            let word_with_char = not_in_wordlist[0];
            let chars: Vec<char> = "~!#$%^&*()-=+[]\\{}:;\"'<>?/0123456789"
                .chars()
                .collect();

            assert!(word_with_char.char_indices().any(|(i, c)| {
                if chars.contains(&c) {
                    let mut word = word_with_char.to_owned();
                    word.remove(i);

                    word_list.contains(&word.as_ref())
                } else {
                    false
                }
            }));
        }
    }
}

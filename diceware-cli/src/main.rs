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

//! The Diceware CLI.

#![warn(rust_2018_idioms)]
#![warn(clippy::redundant_pub_crate)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::use_self)]
#![deny(missing_docs)]
#![deny(unused_must_use)]
#![forbid(unsafe_code)]

use std::process;

use clap::Parser;

use diceware::{Config, EmbeddedList, Error};

/// A Diceware passphrase generator.
#[derive(Debug, Parser)]
#[clap(name = "diceware", author, version)]
struct Cli {
    /// The number of words to generate.
    words: usize,
    /// Use a diceware word file.
    #[clap(long = "file", short = 'f', group = "word_list")]
    word_file: Option<String>,
    /// Use the English embedded word list.
    #[clap(long = "en", group = "word_list")]
    english: bool,
    /// Use the French embedded word list.
    #[clap(long = "fr", group = "word_list")]
    french: bool,
    /// Add a special character to the passphrase.
    #[clap(long, short = 's')]
    with_special_char: bool,
}

fn main() {
    let cli = Cli::parse();

    let config = if let Some(ref filename) = cli.word_file {
        Config::with_filename(filename, cli.words, cli.with_special_char)
    } else {
        let list = if cli.english {
            EmbeddedList::EN
        } else if cli.french {
            EmbeddedList::FR
        } else {
            EmbeddedList::EN
        };

        Config::with_embedded(list, cli.words, cli.with_special_char)
    };

    match diceware::make_passphrase(config) {
        Ok(passphrase) => println!("{}", passphrase),
        Err(err) => {
            match err {
                Error::IO(e) => {
                    let word_file = cli
                        .word_file
                        .expect("IO error without using a word_file.");

                    eprintln!("Error: {}: {}", word_file, e)
                }

                Error::WordList(e) => eprintln!("Error: {}", e),
                Error::NoWords => eprintln!("Error: {}", err),
            }

            process::exit(1);
        }
    };
}

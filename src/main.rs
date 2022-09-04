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
#![warn(clippy::use_self)]
#![deny(missing_docs)]
#![deny(unused_must_use)]
#![forbid(unsafe_code)]

use std::process;

use clap::{App, Arg, ArgGroup};

use diceware::{Config, EmbeddedList, Error};

fn main() {
    let matches = App::new("diceware")
        .version("1.0.1")
        .author("Jean-Philippe Cugnet <jean-philippe@cugnet.eu>")
        .about("A Diceware passphrase generator")
        .arg(
            Arg::with_name("words")
                .help("The number of words to generate")
                .required(true),
        )
        .arg(
            Arg::with_name("word_file")
                .short('f')
                .long("file")
                .takes_value(true)
                .help("Uses a diceware word file"),
        )
        .arg(
            Arg::with_name("english")
                .long("en")
                .help("Uses the English embedded word list"),
        )
        .arg(
            Arg::with_name("french")
                .long("fr")
                .help("Uses the French embedded word list"),
        )
        .group(
            ArgGroup::with_name("word_list")
                .arg("word_file")
                .arg("english")
                .arg("french"),
        )
        .arg(
            Arg::with_name("with-special-char")
                .long("with-special-char")
                .short('s')
                .help("Adds a special character to the passphrase"),
        )
        .get_matches();

    let words_str = matches.value_of("words").unwrap();
    let words = words_str.parse().unwrap_or_else(|_| {
        eprintln!(
            "Error: `{}` is not a valid number of words. Please use an integer \
            instead.",
            words_str
        );

        process::exit(1);
    });

    let word_file = matches.value_of("word_file");
    let with_special_char = matches.is_present("with-special-char");

    let config = if let Some(filename) = word_file {
        Config::with_filename(filename, words, with_special_char)
    } else {
        let list = if matches.is_present("english") {
            EmbeddedList::EN
        } else if matches.is_present("french") {
            EmbeddedList::FR
        } else {
            EmbeddedList::EN
        };

        Config::with_embedded(list, words, with_special_char)
    };

    match diceware::make_passphrase(config) {
        Ok(passphrase) => println!("{}", passphrase),
        Err(err) => {
            match err {
                Error::IO(e) => {
                    eprintln!("Error: {}: {}", word_file.unwrap(), e)
                }

                Error::WordList(e) => eprintln!("Error: {}", e),
                Error::NoWords => eprintln!("Error: {}", err),
            }

            process::exit(1);
        }
    };
}

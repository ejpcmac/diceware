//! A diceware passphrase generator.

#[macro_use]
extern crate clap;
extern crate rand;

mod diceware;

use std::process;
use clap::{App, Arg};
use diceware::Config;

fn main() {
    let matches = App::new("diceware")
        .version("1.0.0")
        .author("Jean-Philippe Cugnet <jean-philippe@cugnet.eu>")
        .about("A diceware passphrase generator")
        .arg(
            Arg::with_name("word_list")
                .help("The diceware word list to use")
                .required(true),
        )
        .arg(
            Arg::with_name("words")
                .help("The number of words to gerenerate")
                .required(true),
        )
        .arg(
            Arg::with_name("with-special-char")
                .short("s")
                .long("with-special-char")
                .help("Adds a special character to the passphrase"),
        )
        .get_matches();

    // Use of `unwrap` is OK since this value is required.
    let filename = matches.value_of("word_list").unwrap();
    let words = value_t_or_exit!(matches, "words", u32);
    let with_special_chars = matches.is_present("with-special-char");

    let config = Config::with_filename(filename, words, with_special_chars);

    match diceware::make_passphrase(config) {
        Ok(passphrase) => println!("{}", passphrase),
        Err(e) => {
            // TODO: Better error messages.
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };
}

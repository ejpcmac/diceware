# Diceware

A Diceware passphrase generator.

## About

[Diceware](http://world.std.com/~reinhold/diceware.html) is a method by Arnold
G. Reinhold for generating passphrases from a dice and a word list.

Although you should not trust your computer’s pseudo-random number generator for
generating strong passphrases, it is sometimes convenient and acceptable to
generate passphrases that are easy to remember, yet less secure than a true
Diceware passphrase.

## Features

This Diceware implementation enables to generate passphrases from a Diceware
word list, with an optional special character inserted at any position in any
word. This differs from the dice version, where the special character can be
inserted only in the six first characters of the six first words.

This implementation embeds two word lists:

* the original Diceware list,
* the French word list from
    [Matthieu Weber](http://weber.fi.eu.org/index.shtml.en#projects), with
    `Église` changed to `Eglise` to avoid encoding and keyboard accessibility
    issues.

In addition to these lists, you can use any other list from a text file
featuring a word by line. A word list **must** contain exactly 7776 unique
words.

Before each passphrase generation, the chosen word list is checked so that you
do not need to trust its creator. This is also the case for embedded lists, so
you do not have to trust me either: just read the source code and acknowledge by
yourself you can use trustless word lists.

## Usage

### As a binary

To install the `diceware` CLI, run:

```sh
$ cargo install --git https://github.com/ejpcmac/diceware.git
```

The simplest way to use the `diceware` binary is to just pass the number of
desired words as an argument:

```sh
$ diceware 8
save andrew liar grater keys chad poetry stole
```

In this case, the embedded original Diceware list is used and no special
character is added.

To add a special character, use the `-s` switch:

```sh
$ diceware -s 8
clerk ion ruddy aid gauss wino listen fl>o
```

To use the embedded French word list, use `--fr`:

```sh
$ diceware --fr 8
jarret papa cv asti brin coron rente don
```

You can also use any external word list:

```sh
$ diceware 8 -f word_list.txt
yah omaha aiken wood noble shoot devil filch
```

### As a library

Add this crate as a dependency to your `Cargo.toml`:

```toml
[dependencies]
diceware = { git = "https://github.com/ejpcmac/diceware.git", tag = "v1.0.1" }
```

#### Example

```rust
use diceware::{Config, EmbeddedList, Error};

// First, generate a config. You can generate one with an embedded list,
// here the English one with 8 words and without a special character:
let config = Config::with_embedded(EmbeddedList::EN, 8, false);

// Alternatively, you can generate a config using an external word list. For
// instance, to generate 6 words from the file `list.txt` with an additional
// special character:
let filename = "list.txt";
let config = Config::with_filename(filename, 8, true);

// Then, try to generate the passphrase:
match diceware::make_passphrase(config) {
    // The happy path: you get your passphrase.
    Ok(passphrase) => println!("{passphrase}"),

    // Some errors can occur:
    Err(err) => {
        match err {
            // IO errors can occur when using an external word list.
            Error::IO(ref e) => eprintln!("Error: {filename}: {e}"),

            // Word list errors can occur if the word list is invalid, i.e.
            // its length is different than 7776 words or it contains
            // duplicates.
            Error::WordList(ref e) => eprintln!("Error: {e}"),

            // No words errors can occur if the number of words to generate
            // is 0.
            Error::NoWords => eprintln!("Error: {err}"),
        }
    }
};
```

## License

Copyright © 2018 Jean-Philippe Cugnet

This project is licensed under the [GNU General Public License 3.0](LICENSE).

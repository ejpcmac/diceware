# Diceware

A Rust implementation of
[Diceware](http://world.std.com/~reinhold/diceware.html). This is a work in
progress, as I am learning Rust.

The French dictionary included in this repository is the one from [Matthieu
Weber](http://weber.fi.eu.org/index.shtml.en#projects), with `Église` changed to
`Eglise` to avoid encoding issues.

## Roadmap

* Have a clean error handling
* Add a command to check the uniqueness of all words in the list
* Embed English and French lists with an auto-check command (this will allow to
  have a all-in-one binary)

# whecto - A Windows clone of hecto
First, this project would not exist without Philipp Flenker's [excellent tutorial](https://www.flenker.blog/hecto/) on creating a text editor in [Rust](https://www.rust-lang.org/).

The current codebase uses [termion](https://crates.io/crates/termion) and the tutorial states that **termion** doesn't support Windows.

This project is an attempt to port the **hecto** editor of the tutorial to Windows using [crossterm](https://crates.io/crates/crossterm).
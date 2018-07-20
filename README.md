# Seven segments

This project contains a simple program that displays number passed as an
argument like it would be on a seven-segments display (SSD).

Here an example of the expected output:

```console
$ ./seven_segments 42
     _ 
|_|  _|
  | |_ 
```

## Getting started

The program is in [Rust](https://www.rust-lang.org). Please make sure to have a
fully working environment before continue.

You can build the binary with cargo:

```console
$ cargo build
```

It generates a binary under `./target/debug/seven_segments`. You can execute
the binary directly or use cargo again:

```console
$ cargo run 42
42
```

In this case, cargo makes sure the binary is up-to-date with the latest version
of the code.

## Running tests

Tests can be run with:

```console
$ cargo test
```

We automatically run test suite against TravisCI when opening a PR. Merging on
`master` requires CI to be green.

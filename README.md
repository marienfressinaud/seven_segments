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
     _ 
|_|  _|
  | |_ 
```

In this case, cargo makes sure the binary is up-to-date with the latest version
of the code.

## Running tests

Tests can be run with:

```console
$ cargo test
```

We automatically run test suite against TravisCI when opening a PR. Merging on
`master` requires CI to be green. We use Rust nightly on the CI in order to be
able to use the `rustfmt` `--check` argument.

## Using Docker

If you prefer to use Docker instead of installing Rust toolchain, a
`Dockerfile` is provided. Its entrypoint is `cargo`.

Please note that we don't recommend to use Docker, prefer to install Rust on
your PC instead.

First, build the image with:

```console
$ docker build -t cargo .
```

Then, you can build the binary with:

```console
$ docker run --rm -v "$(pwd):/code" --user $(id -u) cargo
```

You can override the default command (i.e. `build`):

```console
$ docker run --rm -v "$(pwd):/code" --user $(id -u) cargo run 42
     _ 
|_|  _|
  | |_ 
```

## Using `make` shortcuts

A Makefile is provided to handle common commands:

```console
$ make build # equivalent to `cargo build`
$ make N=42 run # equivalent to `cargo run 42`
$ make test # equivalent to `cargo test`
```

You can use Docker with these three commands by passing a `DOCKER` env var:

```console
$ make DOCKER=yes build
$ make DOCKER=yes N=42 run
$ make DOCKER=yes test
```

A fourth command is provided to build the docker image:

```console
$ make docker-build
```

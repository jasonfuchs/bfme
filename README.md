# BFME

[brainfuck](https://brainfuck.org/) interpreter written in [Rust](https://www.rust-lang.org/).

## But what does it mean?

> **b**rain**f**uck (interpreter written by) **me**.

Nothing else. 👀

## Install

```sh
$ cargo build --release
$ # copy target/release/bfme to PATH
```

## Usage

```sh
$ bfme script.bf # execute script.bf
$ <SOME COMMAND> | bfme - # read from stdin
```

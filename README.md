# gia: Genome Interval Arithmetic

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
![actions status](https://github.com/noamteyssier/gia/workflows/CI/badge.svg)
[![Crates.io](https://img.shields.io/crates/d/gia?color=green&label=crates.io)](https://crates.io/crates/gia)

## Summary

`gia` is a free and open-source command-line tool for highly efficient
and scalable set operations on genomic interval data.

It is inspired by the open source command-line tools [`bedtools`](https://bedtools.readthedocs.io/en/latest/)
and [`bedops`](https://bedops.readthedocs.io/en/latest/) and aims to be a drop-in
replacement to both.

`gia` is written in [rust](https://www.rust-lang.org/) and distributed via [`cargo`](https://rustup.rs/).
It is a command-line tool built on top of [`bedrs`](https://crates.io/crates/bedrs),
a separate and abstracted genomic interval library.

## Installation

`gia` is distributed using the rust package manager `cargo`.

```bash
cargo install gia
```

You can validate the installation by checking `gia`'s help menu:

```bash
gia --help
```

### Installing `cargo`

You can install `cargo` by following the instructions [here](https://rustup.rs/)

## Usage

You can see more detailed usage for each subcommand on the [documentation site](https://noamteyssier.github.io/gia/).

## Issues and Contributions

`gia` is a work-in-progress and under active development by [Noam Teyssier](https://noamteyssier.github.io/about/).

If you are interested in building more functionality or want to
get involved please don't hesitate to reach out!

**Please address all issues to future contributors.**

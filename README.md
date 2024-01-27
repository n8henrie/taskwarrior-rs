# taskwarrior-rs

master: [![master branch build status](https://github.com/n8henrie/taskwarrior-rs/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/n8henrie/taskwarrior-rs/actions/workflows/ci.yml)
<!-- dev: [![dev branch build status](https://github.com/n8henrie/taskwarrior-rs/actions/workflows/ci.yml/badge.svg?branch=dev)](https://github.com/n8henrie/taskwarrior-rs/actions/workflows/ci.yml) -->

[![Crates.io](https://img.shields.io/crates/v/taskwarrior-rs.svg)](https://crates.io/crates/taskwarrior-rs)
[![Docs.rs](https://docs.rs/taskwarrior-rs/badge.svg)](https://docs.rs/taskwarrior-rs)

Rust bindings and a safe wrapper for [taskwarrior].

This is still mostly in the contemplative and pre-alpha stage. I am a rust
novice still learning about FFI and `unsafe`. I don't know C++, and I don't yet
know much about taskwarrior.

Due to improper use of `unsafe`, I'd guess that this project *could* do
terrible things to your system, your taskwarrior database, etc., and I'd
strongly recommend that you don't use it, at least not yet, unless you know
much more about `unsafe` and C++ than I do.

I hope to initially expose much of it as FFI calls into the real taskwarrior,
and slowly migrate parts of it to rust. We'll see.

For anyone looking for a cool rust-based task management tool similar to
taskwarrior, check out
[TaskChampion](https://github.com/taskchampion/taskchampion).

## Roadmap

- [x] Hello, world: `cargo test` runs `taskwarrior list` via FFI
- [ ] ... the rest of the taskwarrior API
- [ ] Safe wrapper

## Features

- TODO

## Introduction

- TODO

## Development

```console
$ git clone https://github.com/n8henrie/taskwarrior-rs.git
$ cd taskwarrior-rs
$ git submodule update --init --recursive
$ cargo test -- --test-threads=1
```

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install taskwarrior-rs`

### Development Setup

1. Clone the repo: `git clone https://github.com/n8henrie/taskwarrior-rs
1. `cd taskwarrior-rs/taskwarrior-sys` or `cd taskwarrior-rs/taskwarrior`
1. `cargo test`

## Configuration

- TODO

## Acknowledgements

- <https://github.com/GothenburgBitFactory/taskwarrior/>
- <https://michael-f-bryan.github.io/rust-ffi-guide/>
- Several helpful and friendly people on <https://users.rust-lang.org>:
  - @MoAlyousef
  - @Michael-F-Bryan
  - @dtolnay
  - Many others!

## Troubleshooting / FAQ

- How can I install an older / specific version of taskwarrior-rs?
    - `cargo install --version x.x`

[taskwarrior]: https://github.com/GothenburgBitFactory/taskwarrior/

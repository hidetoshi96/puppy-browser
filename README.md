# puppy

[![Run tests](https://github.com/lmt-swallow/puppy/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/lmt-swallow/puppy/actions/workflows/test.yml) [![Run lint](https://github.com/lmt-swallow/puppy/actions/workflows/lint.yml/badge.svg?branch=main)](https://github.com/lmt-swallow/puppy/actions/workflows/lint.yml)

`puppy` is an example implementation of a tiny Web browser for educational purposes.

![puppy-browser](./docs/images/puppy-browser.png)

## Build prerequisites

puppy depends on OpenSSL (through `reqwest`), ncurses (the backend of the
`cursive` TUI) and V8 (`rusty_v8`), so the following packages are required to
build it on Linux (tested on Ubuntu 22.04/24.04):

```sh
sudo apt install -y build-essential pkg-config libssl-dev libncurses-dev
```

- `pkg-config` and `libssl-dev`: needed by `openssl-sys`, otherwise its build
  script fails with `Could not find directory of OpenSSL installation`.
- `libncurses-dev`: needed by `ncurses`, otherwise its build script fails with
  `fatal error: ncurses.h: No such file or directory`.

NOTE: `rusty_v8 0.22.1` is not compatible with recent Rust toolchains, because
its `TypeIdHasher` assumes that `std::any::TypeId` is 8 bytes long while
`TypeId` is a 128-bit value since Rust 1.72. puppy therefore vendors a minimally
patched copy of the crate under `vendor/rusty_v8`, which is wired up with
`[patch.crates-io]` in `Cargo.toml`. See
[`vendor/rusty_v8/PATCH.md`](./vendor/rusty_v8/PATCH.md) for the details.

## How to run puppy locally

You can run puppy program with the following command(s):

```sh
cargo run -- help
```

## How to install puppy

You can install puppy by the following command(s):

```sh
cargo install --locked --path . --force
```

After you have successfully installed puppy, you can see help as follows:

```sh
puppy help
```

You can install shell completions as follows:

```sh
# in bash
eval "$(puppy completion bash)"

# in fish
puppy completion fish | source
```

## How to run tests locally

You can run tests with the following command(s):

```sh
cargo test
```

## How to speed up build process

You can cache files related to V8 as follows:

```bash
$ export RUSTY_V8_MIRROR=$HOME/.cache/rusty_v8
(...omitted...)
$ ./scripts/prepare-v8.sh
(...omitted...)
$ cargo build
(...omitted...)
```

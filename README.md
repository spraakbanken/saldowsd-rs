# wsd-rs

A rewrite of [sparv-wsd](https://github.com/spraakbanken/sparv-wsd) in Rust.

## Usage

### Build from source

To build `saldowsd` from source:

```shell
cargo build --release
```

After the build succeeds, you can find the binary in `./target/release/saldowsd`.

## Development

You must have Rust and git-lfs installed.

### Clone the repository

Clone this repo with recursing submodules.

```shell
git clone --recurse-submodules https://github.com/spraakbanken/wsd-rs.git
```

or, if you are using ssh:

```shell
git clone --recurse-submodules git@github.com:spraakbanken/wsd-rs.git
```

or, if using GitHub cli:

```shell
gh repo clone spraakbanken/wsd-rs -- --recurse-submodules
```

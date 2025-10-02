# wsd-rs

A rewrite of [sparv-wsd](https://github.com/spraakbanken/sparv-wsd) in Rust.

## Usage

### Build from source

To build `saldowsd` from source:

```shell
cargo build --release
```

After the build succeeds, you can find the binary in `./target/release/saldowsd`.

### Command-line interface

The supported interface as of today:

```shell
> ./target/release/saldowsd --help
Usage: saldowsd [OPTIONS] --format <FORMAT> <COMMAND>

Commands:
  vector-wsd
  help        Print this message or the help of the given subcommand(s)

Options:
      --format <FORMAT>          Format of the output [possible values: sbxml, tab, eval]
      --batch-size <BATCH_SIZE>  The size of each batch [default: 1]
      --max-sen <MAX_SEN>        The maximum sense [default: 4294967295]
  -v, --verbose...               Verbosity
  -h, --help                     Print help
  -V, --version                  Print version
```

Currently supported command is `vector-wsd`:

```shell
Usage: saldowsd --format <FORMAT> vector-wsd [OPTIONS] --context-width <CONTEXT_WIDTH> --sv-file <SV_FILE> --cv-file <CV_FILE>

Options:
      --decay
      --s1-prior <S1_PRIOR>            [default: 0]
      --context-width <CONTEXT_WIDTH>
      --sv-file <SV_FILE>
      --cv-file <CV_FILE>
  -h, --help                           Print help
  -V, --version                        Print version
```

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

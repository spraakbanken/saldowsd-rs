# saldowsd-rs

[![PyPI version](https://img.shields.io/pypi/v/saldowsd.svg)](https://pypi.org/project/saldowsd/)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/saldowsd.svg)](https://pypi.org/project/saldowsd)

<!--[![PyPI - Downloads](https://img.shields.io/pypi/dm/saldowsd)](https://pypi.org/project/saldowsd/)-->

[![Maturity badge - level 3](https://img.shields.io/badge/Maturity-Level%203%20--%20Stable-green.svg)](https://github.com/spraakbanken/getting-started/blob/main/scorecard.md)
[![Stage](https://img.shields.io/pypi/status/json-arrays)](https://pypi.org/project/json-arrays/)

[![Code Coverage](https://codecov.io/gh/spraakbanken/json-arrays-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/spraakbanken/json-arrays-rs/)

[![CI(check)](https://github.com/spraakbanken/json-arrays-rs/actions/workflows/check.yml/badge.svg)](https://github.com/spraakbanken/json-arrays-rs/actions/workflows/check.yml)
[![CI(release)](https://github.com/spraakbanken/json-arrays-rs/actions/workflows/release.yml/badge.svg)](https://github.com/spraakbanken/json-arrays-rs/actions/workflows/release.yml)
[![CI(scheduled)](https://github.com/spraakbanken/json-arrays-rs/actions/workflows/scheduled.yml/badge.svg)](https://github.com/spraakbanken/json-arrays-rs/actions/workflows/scheduled.yml)
[![CI(test)](https://github.com/spraakbanken/json-arrays-rs/actions/workflows/test.yml/badge.svg)](https://github.com/spraakbanken/json-arrays-rs/actions/workflows/test.yml)

A rewrite of the Java app `saldowsd.jar` in [sparv-wsd](https://github.com/spraakbanken/sparv-wsd) in Rust.

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

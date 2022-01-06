# informatiCup2022

An attempt to solve the railway problem introduced in the [informatiCup2022](https://informaticup.github.io/competition/20-current) using tabu-enhanced genetic search.

## Table Of Contents

-   [Introduction](#introduction)
-   [Usage](#usage)
    -   [Using cargo](#cargo-usage)
    -   [Using Docker](#docker-usage)
    -   [Advance Usage](#advance)
        -   [Tip](#tip)
-   [Tests](#tests)
-   [Benchmarks](#benchmarks)
-   [Coding Style](#coding-style)

<a name="introduction"></a>

## Introduction

This readme contains a usage guide for the program. The theoretical elaboration can be found in the [paper](paper/paper.pdf).

<a name="usage"></a>

## Usage

<a name="cargo-usage"></a>

### Using cargo

Create a build using the following command:

```shell
cargo build --release
```

Then pass the input model via stdin to the the binary:

```shell
cat test-cases/long/input.txt | ./target/release/rstrain
```

<a name="docker-usage"></a>

### Using Docker

You may use the docker container to run the programm.

Create a build first:

```shell
docker build . -t rstrain
```

...and run it via:

```shell
cat test-cases/long/input.txt | docker run --interactive rstrain
```

<a name="advanced"></a>

### Advanced Usage

The advanced usage of the program can be printed via the `--help` flag.

```shell
USAGE:
    rstrain [FLAGS] [OPTIONS]

FLAGS:
    -d, --debug      Prints detailed information about the result
    -p, --plot       Plots the fitness progress, plots are located in ./plots
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --tabu-size <TABU>    Size of tabu list, increase for large models (default 8000000)
    -t, --time <TIME>         Max search duration in milliseconds (default 600000)
    -m, --t-max <TMAX>        The latest time, increase when a solution with a total delay of 0 cannot be found, default
                              value is the latest arrival time of all passengers
```

<a name="tip"></a>

#### Tip

The `--debug` flag comes in handy, when it comes to understanding a the progess and performance of a search process for a model.

For example:

```shell
cat test-cases/long/input.txt | ./target/release/rstrain --debug
```

prints the following output:

```shell
...
+---------------------+---------+
| duration            | 0.440s  |
+---------------------+---------+
| compared moves      | 1365533 |
+---------------------+---------+
| compared moves / ms | 3103    |
+---------------------+---------+
| delays              | 0       |
+---------------------+---------+
| arrived passengers  | 721/721 |
+---------------------+---------+
| t-max               | 6291    |
+---------------------+---------+
```

<a name="tests"></a>

## Tests

Tests can be executed via:

```shell
cargo test
```

<a name="docs"></a>

## Docs

A web version of the program's documentation can be created and opened using
the following command:

```shell
cargo doc --open
```

<a name="benchmarks"></a>

## Benchmarks

> Note: Benchmarks use an unstable features of the rust programming languages, which are currently only available on the nightly channel.

Benchmarks can be run via:

```shell
cargo bench
```

<a name="coding-style"></a>

## Coding Style

Rusts standard code formatter [rustfmt](https://github.com/rust-lang/rustfmt) is used for ensure docing style consistency. It can be run via:

```shell
cargo fmt
```

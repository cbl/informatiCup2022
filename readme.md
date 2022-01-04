# informatiCup22

An attempt to solve the railway problem introduced in the [informatiCup2022](https://informaticup.github.io/competition/20-current) using tabu-enhanced genetic search.

## Introduction

This readme contains a usage guide for the program. The theoretical elaboration can be found in the [paper](paper/paper.pdf).

## Usage

### Using cargo

Create a build using the following command:

```shell
cargo build --release
```

Then pass the input model via stdin to the the binary:

```shell
cat test-cases/long/input.txt | ./target/release/rstrain
```

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

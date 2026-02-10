# Cargo Feature Matrix

Run a cargo command on a matrix of feature sets.

## Motivation

If you've ever wondered if all of your crate's features actually work in
different combinations, then you may be interested in this crate. It looks
up all of your crates features and runs a cargo command for each combination
of features. It also takes transitively enabled features into account. So if
you have `feat-a` that enabled `feat-b`, it won't do two separate runs.

## Install

```shell
cargo install cargo-feature-matrix
```

## Usage

Any cargo command that accepts a `--package`, `--features`, and
`--no-default-features` flag can be run with this command.

```shell
cargo feature-matrix [OPTIONS] <COMMAND> [-- <COMMAND_ARGS_AND_FLAGS>...]
```

### Options

Here is the output of `cargo help feature-matrix`:

```text
USAGE:
    cargo feature-matrix [OPTIONS] <COMMAND> [-- <ARGS>...]

ARGS:
    <COMMAND>
            The cargo commands to run

    <ARGS>...
            Arguments to pass to the cargo command

OPTIONS:
        --color <COLOR>
            Colorize output

            [default: auto]
            [possible values: auto, always, never]

    -d, --deny <DENY>...
            Add these features to the deny list

        --dry-run
            Perform a dry run and print output as if all the jobs succeeded

    -h, --help
            Print help information

    -m, --manifest-path <MANIFEST_PATH>
            The path to the cargo manifest file to use

    -p, --print-jobs
            Print a list of all the cargo commands one per line.

            This is intended to be consumed by external job runners.

    -V, --version
            Print version information
```

### Config

It also supports the following config options in the crate's `Cargo.toml` file.
Every config setting is optional and the config can be omitted entirely if you
are happy with the defaults.

```toml
[package.metadata.feature-matrix]

# If this set is not empty, only these features will be used to construct the
# matrix.
seed = ["a", "list", "of", "features"]

# All of these features will be included in every feature set in the matrix.
include = ["a", "list", "of", "features"]

# Any feature set that includes any of these will be excluded from the matrix.
# This includes features enabled by other features.
#
# This can be used for things like having an "__unstable" feature that gets
# enabled by any other features that use unstable rust features and then
# excluding "__unstable" if not on nightly.
deny = ["a", "list", "of", "features"]

# These sets will be dropped from the matrix.
skip = [["a", "list"], ["of", "feature"], ["lists"]]

# Some crates prepend internal features with a double underscore. If this
# flag is not set, those features will not be used to build the matrix, but
# will be allowed if they are enabled by other features. Default is false.
include_hidden = true

# List sets of features that can't be used together. Any generated feature
# set that is a superset of any of these sets will be dropped from the matrix.
conflict = [["a", "list"], ["of", "feature"], ["lists"]]
```

## License

Licensed under the MIT license.

## Contribution

Any contribution submitted for inclusion in this work shall be licensed as
above without any additional terms and conditions.

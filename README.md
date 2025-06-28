riptree2 (rt)
-------------

![QA Workflow](https://github.com/bcheidemann/riptree2/actions/workflows/qa.yml/badge.svg)
![Crates.io Version](https://img.shields.io/crates/v/riptree2)

riptree2 is a Rust rewrite of the Unix tree command. It aims to be a drop in
replacement (`alias tree="rt --compat"`) with some quality of life improvements,
such as respecting ignore files by default.

# Current Status

riptree2 is under active development. The basic tree functionality has been
implemented, but most options are not supported yet.

# Project Goals

## Backward Compatibility

riptree2 aims to produce identical output to the Unix tree command by default.
We achieve this through a suite of integration tests, which compare the output
of a reference tree implementation to the output of riptree2.

There are some cases where the output of riptree2 will differ slightly from the
output of the Unix tree command. For example, riptree2 automatically respects
ignore files. For maximum backward compatibility, use the `--compat` flag, which
disables all quality of life improvements.

## Performance

![performance comparison of riptree2 with reference implementation](https://bcheidemann.github.io/riptree2/criterion/cli_nested_dirs/report/violin.svg)

The performance of riptree2 should be at least as good as the reference tree
implementation. Currently, riptree2 is approximately 2 to 3 times faster than
reference implementation.

We publish our benchmark results [here](https://bcheidemann.github.io/riptree2/criterion/report/).

# Features

## Nerd Font icons

By default, riptree2 shows Nerd Font icons for each file type. If you don't
have a Nerd Font installed, or prefer not to show icons, you can disable this
feature with the `--no-icons` option.

# Rust API

The Rust API is available for use in other projects, but no guarantee is made
regarding API stability. We may offer a stable public API in future as this
project matures.

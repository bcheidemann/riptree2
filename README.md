riptree2 (rt)
-------------

riptree2 is a Rust rewrite of the Unix tree command. It aims to be a drop in
replacement (`alias tree="rt --compat"`) with some quality of life improvements,
such as automatically respecting ignore files.

# Current Status

riptree2 is under active development. The basic tree functionality has been
implemented, but most options are not supported yet.

# Backward Compatibility

riptree2 aims to produce identical output to the Unix tree command by default.
We achieve this through a suite of integration tests, which compare the output
of a reference tree implementation to the output of riptree2.

There are some cases where the output of riptree2 will differ slightly from the
output of the Unix tree command. For example, riptree2 automatically respects
ignore files. For maximum backward compatibility, use the `--compat` flag, which
disables all quality of life improvements.

# Rust API

The Rust API is available for use in other projects, but no guarantee is made
regarding API stability. We may offer a stable public API in future as this
project matures.

# pdedupe

A small command line utility to dedupe/clean the PATH environment variable.

### Installation

To install, you must have a rust toolchain installed. Using cargo, run `cargo install --git https://github.com/tehbilly/pdedupe`.

### Usage

- `pdedupe` will simply print out the deduplicated contents of `PATH`
- `pdedupe --exists` will do the same but also only include paths that actually exist
- `pdedupe [--exists] ANOTHER_VAR` will do the same but for `ANOTHER_VAR` instead of `PATH`

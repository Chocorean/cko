# cko

Check if a directory is self-contained.

## Usage

```bash
$ cko good_dir
$ echo $?
0
$ cko bad_dir
`outsite_link` is pointing outside of `bad_dir`
$ echo $?
1
```

## Installation

`cargo install cko` (incoming)

## Tests

`cargo test`

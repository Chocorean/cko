# seco

Check if a directory is self-contained.

## Usage

```bash
$ seco good_dir
$ echo $?
0
$ seco bad_dir
`outsite_link` is pointing outside of `bad_dir`
$ echo $?
1
```

## Installation

`cargo install seco`

## Tests

`cargo test`

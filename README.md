# wc

wc is a simple Rust implementation of the Unix command line tool wc,
which is used to display word count and related statistics of a given file or standard input.

## Usage

Installation

Make sure you have Rust installed.
You can then build the project using the following command:

```
cargo build --release
```

Command-line Usage

```
wc [OPTIONS] [FILE]
```

Options:

    -c, --bytes: Print the byte counts.
    -l, --lines: Print the newline counts.
    -w, --words: Print the word counts.
    -m, --chars: Print the character counts.
    [FILE]: Optional. The path to the input file. If not provided, wc reads from standard input.

## Examples

### Count lines, words, and characters in a file

wc -l -w -m path/to/file.txt

### Count bytes in standard input

echo "Hello, world!" | wc -c

## License

This project is licensed under the MIT License.

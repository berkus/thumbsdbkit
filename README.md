ThumbsDBkit
===============

`thumbsdbkit` is a forensic command line tool for analyzing Microsoft Thumbs.db files;

## Installation

Using [cargo](https://crates.io/):

```bash
cargo install thumbsdbkit
```

## Usage

```bash
$ thumbsdbkit --help
ThumbsDBkit 1.0.1


USAGE:
    thumbsdbkit [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    extract    Extract thumbnails
    help       Prints this message or the help of the given subcommand(s)
    ls         List thumbnails
```

### How to list thumbnails: `ls` command

```bash
$ thumbsdbkit help ls
thumbsdbkit-ls
List thumbnails

USAGE:
    thumbsdbkit ls [FLAGS] <FILE>

FLAGS:
        --color      colorize the output
    -d, --details    print more details for each entry
    -h, --help       Prints help information
    -i, --idirid     print the index number of each file
    -V, --version    Prints version information

ARGS:
    <FILE>    Thumbs.db to analyze
```

### How to extract thumbnails: `extract` command

```bash
$ thumbsdbkit help extract
thumbsdbkit-extract
Extract thumbnails

USAGE:
    thumbsdbkit extract <FILE> --out <OUTDIR>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --out <OUTDIR>    Output directory where extracted thumbnails will be stored

ARGS:
    <FILE>    Thumbs.db to analyze
```

## License

<http://www.wtfpl.net/about/>

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
Usage: thumbsdbkit <command> [<args>]

ThumbsDBkit.

Options:
  --help            display usage information

Commands:
  ls                List thumbnails.
  extract           Extract thumbnails.
```

### How to list thumbnails: `ls` command

```bash
$ thumbsdbkit ls --help
Usage: thumbsdbkit ls <file> [-c] [-d] [-i]

List thumbnails.

Positional Arguments:
  file              a Thumbs.db file to analyze

Options:
  -c, --color       colorize the output
  -d, --details     print more details for each entry
  -i, --idirid      print the index number of each file
  --help            display usage information
```

### How to extract thumbnails: `extract` command

```bash
$ thumbsdbkit extract --help
Usage: thumbsdbkit extract <file> [-o <outdir>]

Extract thumbnails.

Positional Arguments:
  file              a Thumbs.db file to analyze

Options:
  -o, --outdir      output directory where extracted thumbnails will be stored
  --help            display usage information
```

## License

<http://www.wtfpl.net/about/>

# Rewind

## Rewinds your filesystem to before you messed up.

Rewind let's you remove all files in the current directory that where
changed/added in the last X seconds.

Very helpful in case of a misplaced git clone, unzip, download or, the worst of
all, yarn install (or any other installer).


## Usage

Base usage:

    $ rewind
    ...
    ...
    will be removed. Continue (yes to continue): _

Prints the files that will be removed and prompts the user to confirm. Only
typing `yes` will result in a rewind. Default rewinds to 60 seconds (see below
for options).

### Flags and options

    -t, --time <seconds> // rewind time
    -f, --force // don't ask to confirm (BE CAREFUL)

See also

    $ rewind --help

## Installation

- From github
    - `git clone https://github.com/debruinf/rewind`
- Get it from crates.io
    - `cargo install rewind`

### Tested environments

tested:

- MacOS

coming soon:

- ubuntu
- debian

## Future development

- Directories
- Recursive rewind
- Rewind window


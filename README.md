[![CICD](https://github.com/BenGale93/cli-diary/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/BenGale93/cli-diary/actions/workflows/rust.yml)
[![Latest version](https://img.shields.io/crates/v/cli-diary.svg)](https://crates.io/crates/cli-diary)

# cli-diary

Keep a diary using the cli. This program is designed for CLI users to quickly
keep a diary or notebook using the command line.

## Installation

Clone this repository using git and run the following command using cargo:

```bash
cargo build --release
```

Then move the resulting binary `./target/release/diary` to `~/.cargo/bin/`.
You will then be able to run the program using the `diary` command.

Alternatively, use cargo to install it from crates.io:

```bash
cargo install cli-diary
```

## First Steps

Setup diary by specifying where you want the diary folder to be kept. Either
`cd` to where you want the `diary/` folder to be created or make a note of the
filepath.  You will then need to use the `init` sub-command as follows:

```bash
diary init <path/to/create>
```

The path is optional and if not provided the new `diary/` folder will be
created in the current directory.

## Configuration

Diary's configuration file is automatically placed in the following location
after you run `init` for the first time:

```bash
~/.config/diary/diary.toml
```

### Content

Below is an example config file.

```toml
# The location of the diary.
diary_path = '/home/user/diary'

# The prefix assigned to the diary entries filename,
# e.g. diary_2020-01-01.md
prefix = 'diary'
```

## Usage

### New Command

The first command you should run each day is the `new` command. This creates a
new entry for the day (only one is currently permitted). If you provide the
`-o` or `--open` flag your editor will open and you will be able to quickly
make your first entry. Save and quit your editor to add the content to the
entry.

```bash
diary new -o
```

### Add Command

The `add` command allows you to add to today's entry on the fly. Similar to
`new -o` the `add` command opens your system editor to allow you to type the
contents on the new entry. To make this easier it is recommended you use a CLI
text editor like nano or vim.

The `add` command also has an optional `--tag` flag which allows you to specify
a Markdown Heading 2 level tag to place above the entry.

```bash
diary new --tag Tip
```

```markdown
## Tip

Content of the new entry.

```

### Open Command

The `open` command allows you to open today's entry for review. Similar to `add`
it will open your system editor, but it will open the entire file.

```bash
diary open
```

To open a different day's entry you can provide the --date tag along with the date
in %Y-%m-%d format, other formats may work.

```bash
dairy open --date 2021-11-01
```

## Diary Folder Structure

The `diary/` folder is organised into monthly sub-folders with each days entry
being a markdown file.

```bash
diary
└── 2021-11
    ├── diary_2021-11-06.md
    └── diary_2021-11-07.md
```

## Acknowledgements

Huge thanks to the authors of the Cargo library. The architecture of this tool
is heavily inspired by it.

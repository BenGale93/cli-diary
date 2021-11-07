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

### Add Command (COMING SOON)

## Diary Folder Structure

The `diary/` folder is organised into monthly sub-folders with each days entry
being a markdown file.

```bash
diary
└── 2021-11
    ├── diary_2021-11-06.md
    └── diary_2021-11-07.md
```

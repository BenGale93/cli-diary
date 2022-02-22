# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - ReleaseDate

## [0.7.0] - 2022-02-22

### Fixed

* Fixed extra "\n" character being added needlessly.

## [0.6.0] - 2022-01-22

### Added

* Added `commit` command. This allows the user to commit entries to a git repo without having
  to navigate to the diary folder.

## [0.5.0] - 2022-01-14

### Added

* Added support for rst diaries. This can be declared when using the `init` function.

### Changed

* Refactored internals to isolate markdown related functions. New structs now deal with
  file type specific operations.
* Functions intended just for testing are placed inside a test submodule.
* Added enum_dispatch as a dependency and changed how different diary entry file types are handled.
* Changed how configs are handled so a specific config can be passed on the CLI.

## [0.4.0] - 2021-12-28

### Changed

* Refactored Config to make use of the builder pattern.

### Added

* Added `get_entry_file` and `get_entry_path` to Config. This allows the caller
  to get the entry using just a date.
* Added prefix flag to the `init` command.
* Added repo flag to the `init` command.

## [0.3.0] - 2021-11-14

### Added

* Added `open` command.

### Changed

* New command errors if diary is unintialised.
* Calls to `edit::edit` now only happen in the binary crate.
  Function provided to `new` and `add` as an argument. Improves testability.

## [0.2.0] - 2021-11-10

### Added

* Added `add` command.

### Changed

* `user_edit_file` now adds an additional line break to the end of the user's text.

## [0.1.0] - 2021-11-07

### Added

* Added `init` and `new` commands.

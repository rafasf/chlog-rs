# chlog-rs [![CircleCI](https://circleci.com/gh/rafasf/chlog-rs.svg?style=svg)](https://circleci.com/gh/rafasf/chlog-rs)

Many projects will have messages with the story identifier as part of the commit message (e.g. `StoryId Commit message here`). In such places, the most valuable part of a changelog is the stories between the changes.

This tools will create a "Story Summary." section listing the unique stories, its titles with a link to the issue tracker followed by a section with the commits that don't belong to any of those and lastly the commits within each of the stories.

## Supported Trackers

### Rally

* Fetches information from `https://rally1.rallydev.com`
* Requires `RALLY_USER` and `RALLY_PWD` environment variables to be available

## Usage

```
USAGE:
    chlog-rs [OPTIONS] --repository <repository path>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <changelog output file name>    The name of the file to be created
    -n, --range <initial-hash..final-hash>     Range of commits to include (using Git style from..to)
    -r, --repository <repository path>         The path to the repository
```

Example

`$ chlog-rs -r ./path-to-repo/.git --file changelog.md --range 06a3ba7..9cea8b4`

## TODO

* [ ] Create Makefile for packaging & eventually publishing
* [ ] Externalise user configuration
* [ ] Externalise application configuration (and find out how to package it)
* [ ] Create trait for changelog writers
* [ ] Be smarter on printing/logging


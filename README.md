# chlog-rs [![CircleCI](https://circleci.com/gh/rafasf/chlog-rs.svg?style=svg)](https://circleci.com/gh/rafasf/chlog-rs)

Many projects will have messages with the story identifier as part of the commit
message (e.g. `StoryId Commit message here`).

This tool will create a "Story Summary." section listing the unique stories, its
titles with a link to the issue tracker followed by a section with the commits
that don't belong to any of those and lastly the commits within each of the
stories.

## Supported Trackers

* Jira
* Rally

## Requirements

* `TRACKER_USER` and `TRACKER_PWD` environment variables to be available

## Usage

```
Changelog 0.1.0

USAGE:
    chlog-rs [OPTIONS] --pattern <pattern regex> --repository <repository path> --tracker <tracker name> --tracker-url <tracker URL>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <changelog output file name>    The name of the file to be created
        --pattern <pattern regex>              The story pattern
    -n, --range <initial-hash..final-hash>     Range of commits to include (using Git style from..to)
    -r, --repository <repository path>         The path to the repository
        --tracker <tracker name>               Inform which tracker to be used [values: jira, rally]
        --tracker-url <tracker URL>            The URL for stories lookup
```

## TODO

* [ ] Create Makefile for packaging & eventually publishing
* [ ] Externalise user configuration
* [ ] Externalise application configuration (and find out how to package it)
* [ ] Create trait for changelog writers
* [ ] Be smarter on printing/logging


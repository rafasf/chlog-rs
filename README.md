# chlog-rs [![CircleCI branch](https://img.shields.io/circleci/project/github/rafasf/chlog-rs/master.svg?style=flat-square)](https://circleci.com/gh/rafasf/chlog-rs) [![License MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](https://github.com/rafasf/chlog-rs/blob/master/LICENSE) [![GitHub release](https://img.shields.io/github/release/rafasf/chlog-rs.svg?style=flat-square)](https://github.com/rafasf/chlog-rs/releases)

Projects that put story identifiers in commits now can generate a Changelog
focused on that. Have it available for the team, quickly craft a message to
share the deployment (or release) changes as well as adding it to the project's
`CHANGELOG.md`.

Besides creating the **Story Summary**, it will have a list of all the commits
that were made for the range provided grouped by its respective tags.

## Supported Trackers

### Requirements

* `TRACKER_USER` and `TRACKER_PWD` environment variables to be available


### Jira

* Appends `/rest/api/latest/issue/` to `<tracker-url>` for story look-up
* Always creates story link using `<tracker-url>` (i.e.
  `<tracker-url>/browse/<story-id>`)

### Rally

* Appends `/slm/webservice/v2.0/hierarchicalrequirement` to `<tracker-url>` for story look-up
* Only creates story link when look-up was successful (i.e.
  `<tracker-url>/#/detail/userstory/<story-id>`)

## General Tags

* `ci`, `ci(component)`
* `chore`, `chore(component)`
* `doc`, `doc(component)`
* `feat`, `feat(component)`
* `refactor`, `refactor(component)`
* `style`, `style(component)`

## Usage

```
Changelog 0.1.0

USAGE:
    chlog [OPTIONS] --repository <repository path>

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


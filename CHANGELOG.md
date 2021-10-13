# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- [#64](https://github.com/adam-mcdaniel/dune/pull/64): Add Changelog
- [#63](https://github.com/adam-mcdaniel/dune/pull/63): Allow builtin operators to be used like symbols

### Fixed
- [#63](https://github.com/adam-mcdaniel/dune/pull/63): Fix parsing of `!` (logical *not*) operator

## [0.1.7] - 2021-10-13

### Added
- [#59](https://github.com/adam-mcdaniel/dune/pull/59): Add recursion depth limit
- [#61](https://github.com/adam-mcdaniel/dune/pull/61): Add builtin `parse` module for parsing JSON, TOML and Dune scripts
- [78ce0fe](https://github.com/adam-mcdaniel/dune/commit/78ce0fe0a3d5e2241978f73cf70f672d79f51612) Add `width` and `height` methods to console module

### Changed
- [#45](https://github.com/adam-mcdaniel/dune/pull/45), [#51](https://github.com/adam-mcdaniel/dune/pull/51): Improve parser error messages and parsing performance
- [#54](https://github.com/adam-mcdaniel/dune/pull/54): Improve syntax highlighting by recovering from tokenizing errors
- [ae50dd0](https://github.com/adam-mcdaniel/dune/commit/ae50dd0fec7da2fa0283754052d26ca7c8910fdd):
    * Change `eval` to never modify the current scope
    * Add `exec` for `eval`'s old behavior
    * A script must now be parsed with `parse@expr` before evaluating it

### Fixed
- [#56](https://github.com/adam-mcdaniel/dune/pull/56): Fix widgets not working correctly on Windows
- [#57](https://github.com/adam-mcdaniel/dune/pull/57): Fix history permissions error
- [#60](https://github.com/adam-mcdaniel/dune/pull/60): Fix incorrect line number 0 in syntax errors

---------

*No changelog available for older releases*

## [0.1.6] - 2019-10-09
## [0.1.5] - 2019-10-05
## [0.1.4] - 2019-10-02
## [0.1.3] - 2021-09-27
## [0.1.2] - 2021-09-27
## [0.1.1] - 2021-09-27
## [0.1.0] - 2019-09-09

[Unreleased]: https://github.com/adam-mcdaniel/dune
[0.1.7]: https://crates.io/crates/dune/0.1.7
[0.1.6]: https://crates.io/crates/dune/0.1.6
[0.1.5]: https://crates.io/crates/dune/0.1.5
[0.1.4]: https://crates.io/crates/dune/0.1.4
[0.1.3]: https://crates.io/crates/dune/0.1.3
[0.1.2]: https://crates.io/crates/dune/0.1.2
[0.1.1]: https://crates.io/crates/dune/0.1.1
[0.1.0]: https://crates.io/crates/dune/0.1.0

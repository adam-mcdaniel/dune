# os_info

**Status:**
[![CI](https://github.com/stanislav-tkach/os_info/workflows/CI/badge.svg)](https://github.com/stanislav-tkach/os_info/actions)
[![Coverage](https://codecov.io/gh/stanislav-tkach/os_info/branch/master/graph/badge.svg)](https://codecov.io/gh/stanislav-tkach/os_info)
[![Dependency status](https://deps.rs/repo/github/stanislav-tkach/os_info/status.svg)](https://deps.rs/repo/github/stanislav-tkach/os_info)

**Project info:**
[![Docs.rs](https://docs.rs/os_info/badge.svg)](https://docs.rs/os_info)
[![Latest version](http://meritbadge.herokuapp.com/os_info)](https://crates.io/crates/os_info)
[![License](https://img.shields.io/github/license/stanislav-tkach/os_info.svg)](https://github.com/stanislav-tkach/os_info/blob/master/LICENSE)

**Project details:**
[![LoC](https://tokei.rs/b1/github/stanislav-tkach/os_info)](https://github.com/stanislav-tkach/os_info)
![Rust 1.41+ required](https://img.shields.io/badge/rust-1.41+-blue.svg?label=Required%20Rust)
![Dependencies](https://api.dependabot.com/badges/status?host=github&repo=stanislav-tkach/os_info)

## Overview

This project consists of two parts: the library that can be used to detect the
operating system type (including version and bitness) and the command line tool
that uses the library.

### Library (`os_info`)

#### `os_info` usage

To use this crate, add `os_info` as a dependency to your project's Cargo.toml:

```toml
[dependencies]
os_info = "3.0"
```

This project has `serde` as an optional dependency, so if you don't need it, then
you can speed up compilation disabling it:

```toml
[dependencies]
os_info = { version = "3.0", default-features = false }
```

#### Example

```rust
let info = os_info::get();

// Print full information:
println!("OS information: {}", info);

// Print information separately:
println!("Type: {}", info.os_type());
println!("Version: {}", info.version());
println!("Bitness: {}", info.bitness());
```

### Command line tool (`os_info_cli`)

A simple wrapper around the `os_info` library.

#### Installation

This tool can be installed using the following cargo command:

```console
cargo install os_info_cli
```

#### `os_info_cli` usage

Despite being named `os_info_cli` during installation, it is actually named
`os_info`. You can use the `--help` flag to see available options:

```console
os_info --help
```

## Supported operating systems

Right now, the following operating system types can be returned:

- Alpine Linux
- Amazon Linux AMI
- Android
- Arch Linux
- CentOS
- Debian
- DragonFly BSD
- Emscripten
- EndeavourOS
- Fedora
- FreeBSD
- Linux
- Linux Mint
- macOS (Mac OS X or OS X)
- Manjaro
- NixOS
- openSUSE
- Oracle Linux
- Pop!_OS
- Raspberry OS
- Red Hat Linux
- Red Hat Enterprise Linux
- Redox
- Solus
- SUSE Linux Enterprise Server
- Ubuntu
- Unknown
- Windows

If you need support for more OS types, I am looking forward to your Pull Request.

## License

`os_info` is licensed under the MIT license. See [LICENSE] for the details.

[lsb_release]: http://refspecs.linuxbase.org/LSB_2.0.1/LSB-PDA/LSB-PDA/lsbrelease.html
[LICENSE]: https://github.com/stanislav-tkach/os_info/blob/master/LICENSE

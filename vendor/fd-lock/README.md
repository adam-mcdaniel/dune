# fd-lock
[![crates.io version][1]][2] 
[![downloads][5]][6] [![docs.rs docs][7]][8]

Advisory cross-platform file locks using file descriptors. Adapted from
[mafintosh/fd-lock].

Note that advisory lock compliance is opt-in, and can freely be ignored by other
parties. This means this crate __should never be used for security purposes__,
but solely to coordinate file access.

[mafintosh/fd-lock]: https://github.com/mafintosh/fd-lock

- [Documentation][8]
- [Crates.io][2]
- [Releases][releases]

## Examples
__Basic usage__
```rust
use fd_lock::RwLock;
use tempfile::tempfile;
use std::io::prelude::*;
use std::fs::File;

fn main() -> Result<(), failure::Error> {
    // Lock a file and write to it.
    let mut f = RwLock::new(tempfile()?);
    f.write()?.write_all(b"chashu cat")?;

    // A lock can also be held across multiple operations.
    let mut f = f.write()?;
    f.write_all(b"nori cat")?;
    f.write_all(b"bird!")?;
    Ok(())
}
```

## Installation
```sh
$ cargo add fd-lock
```

## Safety
This crate uses `unsafe` to interface with `libc` and `winapi`. All invariants
have been carefully checked, and are manually enforced.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

## References
- [LockFile function - WDC](https://docs.microsoft.com/en-us/windows/desktop/api/fileapi/nf-fileapi-lockfile)
- [flock(2) - Linux Man Page](https://linux.die.net/man/2/flock)
- [`libc::flock`](https://docs.rs/libc/0.2.58/libc/fn.flock.html)
- [`winapi::um::fileapi::LockFile`](https://docs.rs/winapi/0.3.7/x86_64-pc-windows-msvc/winapi/um/fileapi/fn.LockFile.html)

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/fd-lock.svg?style=flat-square
[2]: https://crates.io/crates/fd-lock
[3]: https://img.shields.io/travis/yoshuawuyts/fd-lock/master.svg?style=flat-square
[4]: https://travis-ci.org/yoshuawuyts/fd-lock
[5]: https://img.shields.io/crates/d/fd-lock.svg?style=flat-square
[6]: https://crates.io/crates/fd-lock
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/fd-lock

[releases]: https://github.com/yoshuawuyts/fd-lock/releases
[contributing]: https://github.com/yoshuawuyts/fd-lock/blob/master.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/yoshuawuyts/fd-lock/labels/good%20first%20issue
[help-wanted]: https://github.com/yoshuawuyts/fd-lock/labels/help%20wanted

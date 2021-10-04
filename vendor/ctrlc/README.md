# CtrlC
[![Build Status](https://travis-ci.org/Detegr/rust-ctrlc.svg?branch=master)](https://travis-ci.org/Detegr/rust-ctrlc)
[![Build status](https://ci.appveyor.com/api/projects/status/kwg1uu2w2aqn9ta9/branch/master?svg=true)](https://ci.appveyor.com/project/Detegr/rust-ctrlc/branch/master)

A simple easy to use wrapper around Ctrl-C signal.

[Documentation](http://detegr.github.io/doc/ctrlc/)

## Example usage
```rust
use std::sync::mpsc::channel;
use ctrlc;

fn main() {
    let (tx, rx) = channel();
    
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");
    
    println!("Waiting for Ctrl-C...");
    rx.recv().expect("Could not receive from channel.");
    println!("Got it! Exiting..."); 
}
```

#### Try the example yourself
`cargo build --examples && target/debug/examples/readme_example`

## Handling SIGTERM and SIGHUP
Add CtrlC to Cargo.toml using `termination` feature and CtrlC will handle SIGINT, SIGTERM and SIGHUP.
```
[dependencies]
ctrlc = { version = "3.0", features = ["termination"] }
```

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.

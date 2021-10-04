// spell-checker:ignore getconf

use std::fmt::{self, Display, Formatter};
#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "macos",
    target_os = "dragonfly"
))]
use std::process::{Command, Output};

/// Operating system architecture in terms of how many bits compose the basic values it can deal with.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Bitness {
    /// Unknown bitness (unable to determine).
    Unknown,
    /// 32-bit.
    X32,
    /// 64-bit.
    X64,
}

impl Display for Bitness {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Bitness::Unknown => write!(f, "unknown bitness"),
            Bitness::X32 => write!(f, "32-bit"),
            Bitness::X64 => write!(f, "64-bit"),
        }
    }
}

#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "macos"
))]
pub fn get() -> Bitness {
    match &Command::new("getconf").arg("LONG_BIT").output() {
        Ok(Output { stdout, .. }) if stdout == b"32\n" => Bitness::X32,
        Ok(Output { stdout, .. }) if stdout == b"64\n" => Bitness::X64,
        _ => Bitness::Unknown,
    }
}

#[cfg(all(
    test,
    any(target_os = "linux", target_os = "freebsd", target_os = "macos")
))]
mod tests {
    use super::*;
    use pretty_assertions::assert_ne;

    #[test]
    fn get_bitness() {
        let b = get();
        assert_ne!(b, Bitness::Unknown);
    }

    #[test]
    fn display() {
        let data = [
            (Bitness::Unknown, "unknown bitness"),
            (Bitness::X32, "32-bit"),
            (Bitness::X64, "64-bit"),
        ];

        for (bitness, expected) in &data {
            assert_eq!(&bitness.to_string(), expected);
        }
    }
}

use std::fmt::{self, Display, Formatter};

/// A list of supported operating system types.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[non_exhaustive]
pub enum Type {
    /// Alpine Linux (<https://en.wikipedia.org/wiki/Alpine_Linux>).
    Alpine,
    /// Amazon Linux AMI (<https://en.wikipedia.org/wiki/Amazon_Machine_Image#Amazon_Linux_AMI>).
    Amazon,
    /// Android (<https://en.wikipedia.org/wiki/Android_(operating_system)>).
    Android,
    /// Arch Linux (<https://en.wikipedia.org/wiki/Arch_Linux>).
    Arch,
    /// CentOS (<https://en.wikipedia.org/wiki/CentOS>).
    CentOS,
    /// Debian (<https://en.wikipedia.org/wiki/Debian>).
    Debian,
    /// DragonFly BSD (<https://en.wikipedia.org/wiki/DragonFly_BSD>).
    DragonFly,
    /// Emscripten (<https://en.wikipedia.org/wiki/Emscripten>).
    Emscripten,
    /// EndeavourOS (<https://en.wikipedia.org/wiki/EndeavourOS>).
    EndeavourOS,
    /// Fedora (<https://en.wikipedia.org/wiki/Fedora_(operating_system)>).
    Fedora,
    /// FreeBSD (<https://en.wikipedia.org/wiki/FreeBSD>).
    FreeBSD,
    /// Linux based operating system (<https://en.wikipedia.org/wiki/Linux>).
    Linux,
    /// Mac OS X/OS X/macOS (<https://en.wikipedia.org/wiki/MacOS>).
    Macos,
    /// Manjaro (<https://en.wikipedia.org/wiki/Manjaro>).
    Manjaro,
    /// Mint (<https://en.wikipedia.org/wiki/Linux_Mint>).
    Mint,
    /// NixOS (<https://en.wikipedia.org/wiki/NixOS>).
    NixOS,
    /// openSUSE (<https://en.wikipedia.org/wiki/OpenSUSE>).
    openSUSE,
    /// Oracle Linux (<https://en.wikipedia.org/wiki/Oracle_Linux>).
    OracleLinux,
    /// Pop!_OS (<https://en.wikipedia.org/wiki/Pop!_OS>)
    Pop,
    /// Raspberry Pi OS (<https://en.wikipedia.org/wiki/Raspberry_Pi_OS>).
    Raspbian,
    /// Red Hat Linux (<https://en.wikipedia.org/wiki/Red_Hat_Linux>).
    Redhat,
    /// Red Hat Enterprise Linux (<https://en.wikipedia.org/wiki/Red_Hat_Enterprise_Linux>).
    RedHatEnterprise,
    /// Redox (<https://en.wikipedia.org/wiki/Redox_(operating_system)>).
    Redox,
    /// Solus (<https://en.wikipedia.org/wiki/Solus_(operating_system)>).
    Solus,
    /// SUSE Linux Enterprise Server (<https://en.wikipedia.org/wiki/SUSE_Linux_Enterprise>).
    SUSE,
    /// Ubuntu (<https://en.wikipedia.org/wiki/Ubuntu_(operating_system)>).
    Ubuntu,
    /// Unknown operating system.
    Unknown,
    /// Windows (<https://en.wikipedia.org/wiki/Microsoft_Windows>).
    Windows,
}

impl Default for Type {
    fn default() -> Self {
        Type::Unknown
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Type::Alpine => write!(f, "Alpine Linux"),
            Type::Amazon => write!(f, "Amazon Linux AMI"),
            Type::Arch => write!(f, "Arch Linux"),
            Type::DragonFly => write!(f, "DragonFly BSD"),
            Type::Macos => write!(f, "Mac OS"),
            Type::Mint => write!(f, "Linux Mint"),
            Type::Pop => write!(f, "Pop!_OS"),
            Type::Raspbian => write!(f, "Raspberry Pi OS"),
            Type::Redhat => write!(f, "Red Hat Linux"),
            Type::RedHatEnterprise => write!(f, "Red Hat Enterprise Linux"),
            Type::SUSE => write!(f, "SUSE Linux Enterprise Server"),
            _ => write!(f, "{:?}", self),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn default() {
        assert_eq!(Type::Unknown, Type::default());
    }

    #[test]
    fn display() {
        let data = [
            (Type::Alpine, "Alpine Linux"),
            (Type::Amazon, "Amazon Linux AMI"),
            (Type::Android, "Android"),
            (Type::Arch, "Arch Linux"),
            (Type::CentOS, "CentOS"),
            (Type::Debian, "Debian"),
            (Type::DragonFly, "DragonFly BSD"),
            (Type::Emscripten, "Emscripten"),
            (Type::EndeavourOS, "EndeavourOS"),
            (Type::Fedora, "Fedora"),
            (Type::FreeBSD, "FreeBSD"),
            (Type::Linux, "Linux"),
            (Type::Macos, "Mac OS"),
            (Type::Manjaro, "Manjaro"),
            (Type::Mint, "Linux Mint"),
            (Type::NixOS, "NixOS"),
            (Type::openSUSE, "openSUSE"),
            (Type::OracleLinux, "OracleLinux"),
            (Type::Pop, "Pop!_OS"),
            (Type::Raspbian, "Raspberry Pi OS"),
            (Type::Redhat, "Red Hat Linux"),
            (Type::RedHatEnterprise, "Red Hat Enterprise Linux"),
            (Type::Redox, "Redox"),
            (Type::Solus, "Solus"),
            (Type::SUSE, "SUSE Linux Enterprise Server"),
            (Type::Ubuntu, "Ubuntu"),
            (Type::Unknown, "Unknown"),
            (Type::Windows, "Windows"),
        ];

        for (t, expected) in &data {
            assert_eq!(&t.to_string(), expected);
        }
    }
}

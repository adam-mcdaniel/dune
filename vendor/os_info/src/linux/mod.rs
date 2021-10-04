mod file_release;
mod lsb_release;

use log::trace;

use crate::{bitness, Info, Type};

pub fn current_platform() -> Info {
    trace!("linux::current_platform is called");

    let mut info = lsb_release::get()
        .or_else(file_release::get)
        .unwrap_or_else(|| Info::with_type(Type::Linux));
    info.bitness = bitness::get();

    trace!("Returning {:?}", info);
    info
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn os_type() {
        let version = current_platform();
        match version.os_type() {
            Type::Alpine
            | Type::Amazon
            | Type::Arch
            | Type::CentOS
            | Type::Debian
            | Type::EndeavourOS
            | Type::Fedora
            | Type::Linux
            | Type::Manjaro
            | Type::NixOS
            | Type::openSUSE
            | Type::OracleLinux
            | Type::Pop
            | Type::Raspbian
            | Type::Redhat
            | Type::RedHatEnterprise
            | Type::Solus
            | Type::SUSE
            | Type::Ubuntu
            | Type::Mint => (),
            os_type => {
                panic!("Unexpected OS type: {}", os_type);
            }
        }
    }
}

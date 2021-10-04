// spell-checker:ignore codename, noarch, rhel, ootpa, maipo

use std::process::Command;

use log::{debug, trace};

use crate::{matcher::Matcher, Info, Type, Version};

pub fn get() -> Option<Info> {
    let release = retrieve()?;

    let version = match release.version.as_deref() {
        Some("rolling") => Version::Rolling(None),
        Some(v) => Version::Custom(v.to_owned()),
        None => Version::Unknown,
    };

    let os_type = match release.distribution.as_ref().map(String::as_ref) {
        Some("Amazon") | Some("AmazonAMI") => Type::Amazon,
        Some("Arch") => Type::Arch,
        Some("CentOS") => Type::CentOS,
        Some("Debian") => Type::Debian,
        Some("EndeavourOS") => Type::EndeavourOS,
        Some("Fedora") => Type::Fedora,
        Some("Linuxmint") => Type::Mint,
        Some("ManjaroLinux") => Type::Manjaro,
        Some("NixOS") => Type::NixOS,
        Some("openSUSE") => Type::openSUSE,
        Some("OracleServer") => Type::OracleLinux,
        Some("Pop") => Type::Pop,
        Some("Raspbian") => Type::Raspbian,
        Some("RedHatEnterprise") | Some("RedHatEnterpriseServer") => Type::RedHatEnterprise,
        Some("Solus") => Type::Solus,
        Some("SUSE") => Type::SUSE,
        Some("Ubuntu") => Type::Ubuntu,
        _ => Type::Linux,
    };

    Some(Info {
        os_type,
        version,
        codename: release.codename,
        ..Default::default()
    })
}

struct LsbRelease {
    pub distribution: Option<String>,
    pub version: Option<String>,
    pub codename: Option<String>,
}

fn retrieve() -> Option<LsbRelease> {
    match Command::new("lsb_release").arg("-a").output() {
        Ok(output) => {
            trace!("lsb_release command returned {:?}", output);
            Some(parse(&String::from_utf8_lossy(&output.stdout)))
        }
        Err(e) => {
            debug!("lsb_release command failed with {:?}", e);
            None
        }
    }
}

fn parse(output: &str) -> LsbRelease {
    trace!("Trying to parse {:?}", output);

    let distribution = Matcher::PrefixedWord {
        prefix: "Distributor ID:",
    }
    .find(output);

    let codename = Matcher::PrefixedWord {
        prefix: "Codename:",
    }
    .find(output)
    .filter(|c| c != "n/a");

    let version = Matcher::PrefixedVersion { prefix: "Release:" }.find(output);

    trace!(
        "Parsed as '{:?}' distribution and '{:?}' version",
        distribution,
        version
    );

    LsbRelease {
        distribution,
        version,
        codename,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn debian() {
        let parse_results = parse(file());
        assert_eq!(parse_results.distribution, Some("Debian".to_string()));
        assert_eq!(parse_results.version, Some("7.8".to_string()));
        assert_eq!(parse_results.codename, Some("wheezy".to_string()));
    }

    #[test]
    fn arch() {
        let parse_results = parse(arch_file());
        assert_eq!(parse_results.distribution, Some("Arch".to_string()));
        assert_eq!(parse_results.version, Some("rolling".to_string()));
        assert_eq!(parse_results.codename, None);
    }

    #[test]
    fn fedora() {
        let parse_results = parse(fedora_file());
        assert_eq!(parse_results.distribution, Some("Fedora".to_string()));
        assert_eq!(parse_results.version, Some("26".to_string()));
        assert_eq!(parse_results.codename, Some("TwentySix".to_string()));
    }

    #[test]
    fn ubuntu() {
        let parse_results = parse(ubuntu_file());
        assert_eq!(parse_results.distribution, Some("Ubuntu".to_string()));
        assert_eq!(parse_results.version, Some("16.04".to_string()));
        assert_eq!(parse_results.codename, Some("xenial".to_string()));
    }

    #[test]
    fn mint() {
        let parse_results = parse(mint_file());
        assert_eq!(parse_results.distribution, Some("Linuxmint".to_string()));
        assert_eq!(parse_results.version, Some("20".to_string()));
        assert_eq!(parse_results.codename, Some("ulyana".to_string()));
    }

    #[test]
    fn nixos() {
        let parse_results = parse(nixos_file());
        assert_eq!(parse_results.distribution, Some("NixOS".to_string()));
        assert_eq!(
            parse_results.version,
            Some("21.05pre275822.916ee862e87".to_string())
        );
        assert_eq!(parse_results.codename, Some("okapi".to_string()));
    }

    #[test]
    fn amazon1() {
        let parse_results = parse(amazon1_file());
        assert_eq!(parse_results.distribution, Some("AmazonAMI".to_string()));
        assert_eq!(parse_results.version, Some("2018.03".to_string()));
        assert_eq!(parse_results.codename, None);
    }

    #[test]
    fn amazon2() {
        let parse_results = parse(amazon2_file());
        assert_eq!(parse_results.distribution, Some("Amazon".to_string()));
        assert_eq!(parse_results.version, Some("2".to_string()));
        assert_eq!(parse_results.codename, Some("Karoo".to_string()));
    }

    #[test]
    fn redhat_enterprise_8() {
        let parse_results = parse(rhel8_file());
        assert_eq!(
            parse_results.distribution,
            Some("RedHatEnterprise".to_string())
        );
        assert_eq!(parse_results.version, Some("8.1".to_string()));
        assert_eq!(parse_results.codename, Some("Ootpa".to_string()));
    }

    #[test]
    fn redhat_enterprise_7() {
        let parse_results = parse(rhel7_file());
        assert_eq!(
            parse_results.distribution,
            Some("RedHatEnterpriseServer".to_string())
        );
        assert_eq!(parse_results.version, Some("7.7".to_string()));
        assert_eq!(parse_results.codename, Some("Maipo".to_string()));
    }

    #[test]
    fn redhat_enterprise_6() {
        let parse_results = parse(rhel6_file());
        assert_eq!(
            parse_results.distribution,
            Some("RedHatEnterpriseServer".to_string())
        );
        assert_eq!(parse_results.version, Some("6.10".to_string()));
        assert_eq!(parse_results.codename, Some("Santiago".to_string()));
    }

    #[test]
    fn suse_enterprise_15_1() {
        let parse_results = parse(suse_enterprise15_1_file());
        assert_eq!(parse_results.distribution, Some("SUSE".to_string()));
        assert_eq!(parse_results.version, Some("15.1".to_string()));
        assert_eq!(parse_results.codename, None);
    }

    #[test]
    fn suse_enterprise_12_5() {
        let parse_results = parse(suse_enterprise12_5_file());
        assert_eq!(parse_results.distribution, Some("SUSE".to_string()));
        assert_eq!(parse_results.version, Some("12.5".to_string()));
        assert_eq!(parse_results.codename, None);
    }

    #[test]
    fn open_suse_15_1() {
        let parse_results = parse(open_suse_15_1_file());
        assert_eq!(parse_results.distribution, Some("openSUSE".to_string()));
        assert_eq!(parse_results.version, Some("15.1".to_string()));
        assert_eq!(parse_results.codename, None);
    }

    #[test]
    fn oracle_linux_7_5() {
        let parse_results = parse(oracle_server_linux_7_5_file());
        assert_eq!(parse_results.distribution, Some("OracleServer".to_string()));
        assert_eq!(parse_results.version, Some("7.5".to_string()));
        assert_eq!(parse_results.codename, None);
    }

    #[test]
    fn oracle_linux_8_1() {
        let parse_results = parse(oracle_server_linux_8_1_file());
        assert_eq!(parse_results.distribution, Some("OracleServer".to_string()));
        assert_eq!(parse_results.version, Some("8.1".to_string()));
        assert_eq!(parse_results.codename, None);
    }

    #[test]
    fn pop_os_20_04_lts() {
        let parse_results = parse(pop_os_20_04_lts_file());
        assert_eq!(parse_results.distribution, Some("Pop".to_string()));
        assert_eq!(parse_results.version, Some("20.04".to_string()));
        assert_eq!(parse_results.codename, Some("focal".to_string()));
    }

    #[test]
    fn solus_4_1() {
        let parse_results = parse(solus_4_1_file());
        assert_eq!(parse_results.distribution, Some("Solus".to_string()));
        assert_eq!(parse_results.version, Some("4.1".to_string()));
        assert_eq!(parse_results.codename, Some("fortitude".to_string()));
    }

    #[test]
    fn manjaro() {
        let parse_results = parse(manjaro_19_0_2_file());
        assert_eq!(parse_results.distribution, Some("ManjaroLinux".to_string()));
        assert_eq!(parse_results.version, Some("19.0.2".to_string()));
        assert_eq!(parse_results.codename, None);
    }

    #[test]
    fn endeavouros() {
        let parse_results = parse(endeavouros_file());
        assert_eq!(parse_results.distribution, Some("EndeavourOS".to_string()));
        assert_eq!(parse_results.version, Some("rolling".to_string()));
        assert_eq!(parse_results.codename, None);
    }

    #[test]
    fn raspbian() {
        let parse_results = parse(raspberry_os_file());
        assert_eq!(parse_results.distribution, Some("Raspbian".to_string()));
        assert_eq!(parse_results.version, Some("10".to_string()));
        assert_eq!(parse_results.codename, None);
    }

    fn file() -> &'static str {
        "\nDistributor ID:	Debian\n\
         Description:	Debian GNU/Linux 7.8 (wheezy)\n\
         Release:	7.8\n\
         Codename:	wheezy\n\
         "
    }

    fn arch_file() -> &'static str {
        "\nLSB Version:	1.4\n\
         Distributor ID:	Arch\n\
         Description:	Arch Linux\n\
         Release:	rolling\n\
         Codename:	n/a"
    }

    fn fedora_file() -> &'static str {
        "\nLSB Version:    :core-4.1-amd64:core-4.1-noarch:cxx-4.1-amd64:cxx-4.1-noarch\n\
         Distributor ID: Fedora\n\
         Description:    Fedora release 26 (Twenty Six)\n\
         Release:    26\n\
         Codename:   TwentySix\n\
         "
    }

    fn ubuntu_file() -> &'static str {
        "Distributor ID: Ubuntu\n\
         Description:    Ubuntu 16.04.5 LTS\n\
         Release:        16.04\n\
         Codename:       xenial"
    }

    fn mint_file() -> &'static str {
        "Distributor ID:	Linuxmint\n\
         Description:	    Linux Mint 20\n\
         Release:	        20\n\
         Codename:	        ulyana"
    }

    fn nixos_file() -> &'static str {
        "Distributor ID: NixOS\n\
         Description:    NixOS 21.05 (Okapi)\n\
         Release:        21.05pre275822.916ee862e87\n\
         Codename:       okapi"
    }

    // Amazon Linux 1 uses a separate Distributor ID and Release format from Amazon Linux 2
    fn amazon1_file() -> &'static str {
        "LSB Version:	:base-4.0-amd64:base-4.0-noarch:core-4.0-amd64:core-4.0-noarch\n\
         Distributor ID:	AmazonAMI\n\
         Description:	Amazon Linux AMI release 2018.03\n\
         Release:	2018.03\n\
         Codename:	n/a\n\
         "
    }

    // Amazon Linux 2 uses a separate Distributor ID and Release format from Amazon Linux 1
    fn amazon2_file() -> &'static str {
        "LSB Version:	:core-4.1-amd64:core-4.1-noarch\n\
         Distributor ID:	Amazon\n\
         Description:	Amazon Linux release 2 (Karoo)\n\
         Release:	2\n\
         Codename:	Karoo\n\
         "
    }

    fn rhel8_file() -> &'static str {
        "LSB Version:	:core-4.1-amd64:core-4.1-noarch\n\
         Distributor ID:	RedHatEnterprise\n\
         Description:	Red Hat Enterprise Linux release 8.1 (Ootpa)\n\
         Release:	8.1\n\
         Codename:	Ootpa\n\
         "
    }

    fn rhel7_file() -> &'static str {
        "LSB Version:	:core-4.1-amd64:core-4.1-noarch\n\
         Distributor ID:	RedHatEnterpriseServer\n\
         Description:	Red Hat Enterprise Linux Server release 7.7 (Maipo)\n\
         Release:	7.7\n\
         Codename:	Maipo\n\
         "
    }

    fn rhel6_file() -> &'static str {
        "LSB Version:	:base-4.0-amd64:base-4.0-noarch:core-4.0-amd64:core-4.0-noarch:graphics-4.0-amd64:graphics-4.0-noarch:printing-4.0-amd64:printing-4.0-noarch\n\
        Distributor ID:	RedHatEnterpriseServer\n\
        Description:	Red Hat Enterprise Linux Server release 6.10 (Santiago)\n\
        Release:	6.10\n\
        Codename:	Santiago\n\
        "
    }

    fn suse_enterprise15_1_file() -> &'static str {
        "LSB Version:	n/a\n\
        Distributor ID:	SUSE\n\
        Description:	SUSE Linux Enterprise Server 15 SP1\n\
        Release:	15.1\n\
        Codename:	n/a\n\
        "
    }

    fn suse_enterprise12_5_file() -> &'static str {
        "LSB Version:	n/a\n\
        Distributor ID:	SUSE\n\
        Description:	SUSE Linux Enterprise Server 12 SP5\n\
        Release:	12.5\n\
        Codename:	n/a\n\
        "
    }

    fn raspberry_os_file() -> &'static str {
        "LSB Version:   n/a\n\
         Distributor ID: Raspbian\n\
         Description:    Raspbian GNU/Linux 10 (buster)\n\
         Release:        10\n\
         Codename:       n/a\n\
         "
    }

    fn open_suse_15_1_file() -> &'static str {
        "LSB Version:	n/a\n\
        Distributor ID:	openSUSE\n\
        Description:	openSUSE Leap 15.1\n\
        Release:	15.1\n\
        Codename:	n/a\n\
        "
    }

    fn oracle_server_linux_7_5_file() -> &'static str {
        "LSB Version:	:core-4.1-amd64:core-4.1-noarch\n\
        Distributor ID:	OracleServer\n\
        Description:	Oracle Linux Server release 7.5\n\
        Release:	7.5\n\
        Codename:	n/a\n\
        "
    }

    fn oracle_server_linux_8_1_file() -> &'static str {
        "LSB Version:	:core-4.1-amd64:core-4.1-noarch\n\
        Distributor ID:	OracleServer\n\
        Description:	Oracle Linux Server release 8.1\n\
        Release:	8.1\n\
        Codename:	n/a\n\
        "
    }

    fn pop_os_20_04_lts_file() -> &'static str {
        "No LSB modules are available.\n\
        Distributor ID: Pop\n\
        Description: Pop!_OS 20.04 LTS\n\
        Release: 20.04\n\
        Codename: focal\n\
        "
    }

    fn solus_4_1_file() -> &'static str {
        "LSB Version:	1.4\n\
        Distributor ID:	Solus\n\
        Description:	Solus\n\
        Release:	4.1\n\
        Codename:	fortitude\n\
        "
    }

    fn manjaro_19_0_2_file() -> &'static str {
        "LSB Version:    n/a\n\
        Distributor ID: ManjaroLinux\n\
        Description:    Manjaro Linux\n\
        Release:        19.0.2\n\
        Codename:       n/a\n\
        "
    }

    fn endeavouros_file() -> &'static str {
        "LSB Version:	1.4\n\
        Distributor ID:	EndeavourOS\n\
        Description:	EndeavourOS Linux\n\
        Release:	rolling\n\
        Codename:	n/a\n\
        "
    }
}

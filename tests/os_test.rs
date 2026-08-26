use vps_config::os::detect::{Distro, parse_os_release};

#[test]
fn test_parse_ubuntu() {
    let content = r#"NAME="Ubuntu"
VERSION="22.04.3 LTS (Jammy Jellyfish)"
ID=ubuntu
ID_LIKE=debian"#;
    let distro = parse_os_release(content).unwrap();
    assert_eq!(distro, Distro::Ubuntu);
}

#[test]
fn test_parse_fedora() {
    let content = r#"NAME="Fedora Linux"
VERSION="39 (Workstation Edition)"
ID=fedora"#;
    let distro = parse_os_release(content).unwrap();
    assert_eq!(distro, Distro::Fedora);
}

#[test]
fn test_parse_debian() {
    let content = r#"NAME="Debian GNU/Linux"
VERSION="12 (bookworm)"
ID=debian"#;
    let distro = parse_os_release(content).unwrap();
    assert_eq!(distro, Distro::Debian);
}

#[test]
fn test_parse_rhel() {
    let content = r#"NAME="Red Hat Enterprise Linux"
VERSION="9.2 (Plow)"
ID=rhel"#;
    let distro = parse_os_release(content).unwrap();
    assert_eq!(distro, Distro::RHEL);
}

#[test]
fn test_parse_centos_mapped_to_rhel() {
    let content = r#"NAME="CentOS Linux"
VERSION="7 (Core)"
ID=centos"#;
    let distro = parse_os_release(content).unwrap();
    assert_eq!(distro, Distro::RHEL);
}

#[test]
fn test_parse_rocky_mapped_to_rhel() {
    let content = r#"NAME="Rocky Linux"
VERSION="9.3 (Blue Onyx)"
ID=rocky"#;
    let distro = parse_os_release(content).unwrap();
    assert_eq!(distro, Distro::RHEL);
}

#[test]
fn test_parse_unsupported_distro() {
    let content = r#"NAME="Alpine Linux"
VERSION="3.18.0"
ID=alpine"#;
    let result = parse_os_release(content);
    assert!(result.is_err());
}

#[test]
fn test_parse_missing_id() {
    let content = r#"NAME="Unknown Linux"
VERSION="1.0"#;
    let result = parse_os_release(content);
    assert!(result.is_err());
}

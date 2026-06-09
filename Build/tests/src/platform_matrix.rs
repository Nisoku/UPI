use upi_core::{detect, Command, OsType, PlatformConfig, PlatformRegistry, Resolver};

fn known_types() -> Vec<OsType> {
    vec![
        OsType::Macos,
        OsType::Debian,
        OsType::Ubuntu,
        OsType::Mint,
        OsType::Pop,
        OsType::Fedora,
        OsType::Arch,
        OsType::Manjaro,
        OsType::Windows,
    ]
}

#[test]
fn load_all_configs() {
    let registry = PlatformRegistry::load().unwrap();
    assert!(registry.all().len() >= 5);
}

#[test]
fn every_known_type_has_a_config() {
    let registry = PlatformRegistry::load().unwrap();
    for t in known_types() {
        let config = registry.for_type(&t);
        assert!(config.is_some(), "no config for {t:?}");
    }
}

#[test]
fn configs_have_required_fields() {
    let registry = PlatformRegistry::load().unwrap();
    for config in registry.all() {
        assert!(!config.manager.is_empty());
        assert!(!config.targets.is_empty());
        assert!(
            config.install.contains("{package}"),
            "{}: install missing {{{{package}}}}",
            config.manager
        );
    }
}

#[test]
fn macos_resolves_to_brew() {
    let registry = PlatformRegistry::load().unwrap();
    let config = registry.for_type(&OsType::Macos).unwrap();
    assert_eq!(config.manager, "homebrew");
}

#[test]
fn command_from_config_no_sudo() {
    let config = PlatformConfig {
        targets: vec![OsType::Macos],
        manager: "homebrew".into(),
        sudo: false,
        install: "brew install {package}".into(),
        search: None,
        provides: None,
        provides_parse: None,
        binary_paths: vec!["/opt/homebrew/bin".into()],
    };

    let cmd = Command::from_config(&config, "ffmpeg");
    assert_eq!(cmd.to_display(), "brew install ffmpeg");
}

#[test]
fn command_from_config_with_sudo() {
    let config = PlatformConfig {
        targets: vec![OsType::Debian],
        manager: "apt".into(),
        sudo: true,
        install: "apt install -y {package}".into(),
        search: None,
        provides: None,
        provides_parse: None,
        binary_paths: vec!["/usr/bin".into()],
    };

    let cmd = Command::from_config(&config, "ffmpeg");
    assert_eq!(cmd.to_display(), "sudo apt install -y ffmpeg");
}

#[test]
fn command_from_config_multiple_packages() {
    let config = PlatformConfig {
        targets: vec![OsType::Arch],
        manager: "pacman".into(),
        sudo: true,
        install: "pacman -S --noconfirm {package}".into(),
        search: None,
        provides: None,
        provides_parse: None,
        binary_paths: vec!["/usr/bin".into()],
    };

    let cmd = Command::from_config(&config, "vim");
    assert_eq!(cmd.to_display(), "sudo pacman -S --noconfirm vim");
}

#[test]
fn resolver_detects_and_resolves() {
    let resolver = Resolver::new().unwrap();
    let cmd = resolver.resolve("ffmpeg").unwrap();
    let display = cmd.to_display();
    assert!(
        display.contains("ffmpeg"),
        "expected ffmpeg in command, got: {display}"
    );
}

#[test]
fn detect_returns_expected_type() {
    let os_type = detect();
    assert!(
        known_types().contains(&os_type),
        "unexpected OS type: {os_type:?}"
    );
}

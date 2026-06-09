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
fn search_candidates_returns_results() {
    let resolver = Resolver::new().unwrap();
    let os_type = detect();

    let candidates = resolver.search_candidates("python", &os_type).unwrap();
    assert!(
        !candidates.is_empty(),
        "expected at least one candidate for 'python'"
    );
    let names: Vec<&str> = candidates.iter().map(|c| c.name.as_str()).collect();
    assert!(
        names.contains(&"python"),
        "expected 'python' in candidates, got: {names:?}"
    );
}

#[test]
fn search_candidates_partial_match() {
    let resolver = Resolver::new().unwrap();
    let os_type = detect();

    let candidates = resolver.search_candidates("pyt", &os_type).unwrap();
    assert!(
        !candidates.is_empty(),
        "expected at least one candidate for 'pyt'"
    );
    let sources: Vec<&str> = candidates.iter().map(|c| c.source.as_str()).collect();
    assert!(
        sources.iter().any(|s| s.starts_with("database")),
        "expected a DB result in candidates, got sources: {sources:?}"
    );
}

#[test]
fn search_candidates_always_includes_identity() {
    let resolver = Resolver::new().unwrap();
    let os_type = detect();

    let candidates = resolver
        .search_candidates("zzz_nonexistent_zzz", &os_type)
        .unwrap();
    let names: Vec<&str> = candidates.iter().map(|c| c.name.as_str()).collect();
    assert!(
        names.contains(&"zzz_nonexistent_zzz"),
        "expected identity fallback in candidates, got: {names:?}"
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

#[test]
fn repo_to_os_maps_homebrew() {
    let registry = PlatformRegistry::load().unwrap();
    let result = registry.repo_to_os("homebrew");
    assert_eq!(result, Some(&OsType::Macos));
}

#[test]
fn repo_to_os_maps_debian_versioned() {
    let registry = PlatformRegistry::load().unwrap();
    let result = registry.repo_to_os("debian_13");
    assert_eq!(result, Some(&OsType::Debian));
}

#[test]
fn repo_to_os_maps_ubuntu_versioned() {
    let registry = PlatformRegistry::load().unwrap();
    let result = registry.repo_to_os("ubuntu_24_04");
    assert_eq!(result, Some(&OsType::Ubuntu));
}

#[test]
fn repo_to_os_maps_linuxmint() {
    let registry = PlatformRegistry::load().unwrap();
    let result = registry.repo_to_os("linuxmint");
    assert_eq!(result, Some(&OsType::Mint));
}

#[test]
fn repo_to_os_maps_arch() {
    let registry = PlatformRegistry::load().unwrap();
    let result = registry.repo_to_os("arch");
    assert_eq!(result, Some(&OsType::Arch));
}

#[test]
fn repo_to_os_maps_manjaro_stable() {
    let registry = PlatformRegistry::load().unwrap();
    let result = registry.repo_to_os("manjaro_stable");
    assert_eq!(result, Some(&OsType::Manjaro));
}

#[test]
fn repo_to_os_maps_fedora_versioned() {
    let registry = PlatformRegistry::load().unwrap();
    let result = registry.repo_to_os("fedora_40");
    assert_eq!(result, Some(&OsType::Fedora));
}

#[test]
fn repo_to_os_maps_winget() {
    let registry = PlatformRegistry::load().unwrap();
    let result = registry.repo_to_os("winget");
    assert_eq!(result, Some(&OsType::Windows));
}

#[test]
fn repo_to_os_maps_chocolatey() {
    let registry = PlatformRegistry::load().unwrap();
    let result = registry.repo_to_os("chocolatey");
    assert_eq!(result, Some(&OsType::Windows));
}

#[test]
fn repo_to_os_maps_macports() {
    let registry = PlatformRegistry::load().unwrap();
    let result = registry.repo_to_os("macports");
    assert_eq!(result, Some(&OsType::Macos));
}

#[test]
fn repo_to_os_maps_scoop() {
    let registry = PlatformRegistry::load().unwrap();
    let result = registry.repo_to_os("scoop");
    assert_eq!(result, Some(&OsType::Windows));
}

#[test]
fn repo_to_os_unknown_repo_returns_none() {
    let registry = PlatformRegistry::load().unwrap();
    let result = registry.repo_to_os("freebsd");
    assert_eq!(result, None);
}

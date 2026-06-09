#[cfg(test)]
mod platform_tests {
    use upi_core::{OsType, PlatformRegistry};

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
}

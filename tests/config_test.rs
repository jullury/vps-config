use vps_config::config::loader::load_config;
use std::path::Path;

#[test]
fn test_load_example_config() {
    let config = load_config(Path::new("config.example.toml")).unwrap();
    assert_eq!(config.vps.port, 22);
    assert!(config.services.docker);
}

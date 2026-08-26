use vps_config::config::schema::VpsConfig;
use vps_config::ssh::client::SshClient;
use vps_config::ssh::executor::Executor;

#[test]
fn test_ssh_client_creation() {
    let config = VpsConfig {
        ip: "127.0.0.1".to_string(),
        port: 22,
        user: "root".to_string(),
        auth: "key".to_string(),
        key_path: Some("~/.ssh/id_rsa".to_string()),
    };
    let client = SshClient::new(&config);
    assert!(client.is_ok());
}

#[test]
fn test_ssh_client_stores_config() {
    let config = VpsConfig {
        ip: "192.168.1.100".to_string(),
        port: 2222,
        user: "deploy".to_string(),
        auth: "key".to_string(),
        key_path: Some("~/.ssh/deploy_key".to_string()),
    };
    let client = SshClient::new(&config).unwrap();
    let retrieved = client.config();
    assert_eq!(retrieved.ip, "192.168.1.100");
    assert_eq!(retrieved.port, 2222);
    assert_eq!(retrieved.user, "deploy");
}

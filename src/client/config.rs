use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

use onc_rpc_rs::auth::AuthSysParams;

/// NFS protocol minor version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NfsMinorVersion {
    V4_0,
    V4_1,
}

/// Server endpoint for NFS.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerAddr {
    pub ip: IpAddr,
    pub port: u16,
}

impl ServerAddr {
    pub fn new(ip: IpAddr, port: u16) -> Self {
        Self { ip, port }
    }

    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.ip, self.port)
    }
}

/// Authentication settings for the connection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthConfig {
    AuthNone,
    AuthSys(AuthSysParams),
    // Future: support for other auth flavors (e.g., Kerberos)
}

/// Connection and protocol settings for LowLevelClient.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LowLevelClientConfig {
    pub server: ServerAddr,
    pub minor_version: NfsMinorVersion,
    pub auth: AuthConfig,
    pub rpc_timeout: Duration,
    pub connect_timeout: Duration,
    pub max_recv_size: usize,
}

impl Default for LowLevelClientConfig {
    fn default() -> Self {
        Self {
            server: ServerAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 2049),
            minor_version: NfsMinorVersion::V4_1,
            auth: AuthConfig::AuthNone,
            rpc_timeout: Duration::from_secs(30),
            connect_timeout: Duration::from_secs(5),
            max_recv_size: 1024 * 1024,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    #[test]
    fn test_serveraddr_new() {
        let addr = ServerAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 10)), 2049);

        assert_eq!(addr.ip, IpAddr::V4(Ipv4Addr::new(192, 168, 1, 10)));
        assert_eq!(addr.port, 2049);
    }

    #[test]
    fn test_serveraddr_socket_addr() {
        let addr = ServerAddr::new(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)), 2049);

        assert_eq!(
            addr.socket_addr(),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)), 2049)
        );
    }

    #[test]
    fn test_nfsminorversion_equality() {
        assert_eq!(NfsMinorVersion::V4_0, NfsMinorVersion::V4_0);
        assert_eq!(NfsMinorVersion::V4_1, NfsMinorVersion::V4_1);
        assert_ne!(NfsMinorVersion::V4_0, NfsMinorVersion::V4_1);
    }

    #[test]
    fn test_authconfig_authnone_equality() {
        assert_eq!(AuthConfig::AuthNone, AuthConfig::AuthNone);
    }

    #[test]
    fn test_authconfig_authsys_equality() {
        let a = AuthConfig::AuthSys(AuthSysParams {
            stamp: 1,
            machinename: "client1".to_string(),
            uid: 1000,
            gid: 100,
            gids: vec![100, 200],
        });

        let b = AuthConfig::AuthSys(AuthSysParams {
            stamp: 1,
            machinename: "client1".to_string(),
            uid: 1000,
            gid: 100,
            gids: vec![100, 200],
        });

        assert_eq!(a, b);
    }

    #[test]
    fn test_lowlevelclientconfig_default() {
        let config = LowLevelClientConfig::default();

        assert_eq!(
            config.server,
            ServerAddr::new("127.0.0.1".parse().unwrap(), 2049)
        );
        assert_eq!(config.minor_version, NfsMinorVersion::V4_1);
        assert_eq!(config.auth, AuthConfig::AuthNone);
        assert_eq!(config.rpc_timeout, Duration::from_secs(30));
        assert_eq!(config.connect_timeout, Duration::from_secs(5));
        assert_eq!(config.max_recv_size, 1024 * 1024);
    }

    #[test]
    fn test_lowlevelclientconfig_custom() {
        let config = LowLevelClientConfig {
            server: ServerAddr::new("192.168.0.10".parse().unwrap(), 2050),
            minor_version: NfsMinorVersion::V4_0,
            auth: AuthConfig::AuthSys(AuthSysParams {
                stamp: 99,
                machinename: "tester".to_string(),
                uid: 1234,
                gid: 5678,
                gids: vec![1, 2, 3],
            }),
            rpc_timeout: Duration::from_secs(10),
            connect_timeout: Duration::from_secs(2),
            max_recv_size: 4096,
        };

        assert_eq!(
            config.server,
            ServerAddr::new("192.168.0.10".parse().unwrap(), 2050)
        );
        assert_eq!(config.minor_version, NfsMinorVersion::V4_0);
        assert_eq!(config.rpc_timeout, Duration::from_secs(10));
        assert_eq!(config.connect_timeout, Duration::from_secs(2));
        assert_eq!(config.max_recv_size, 4096);

        match config.auth {
            AuthConfig::AuthSys(params) => {
                assert_eq!(params.stamp, 99);
                assert_eq!(params.machinename, "tester");
                assert_eq!(params.uid, 1234);
                assert_eq!(params.gid, 5678);
                assert_eq!(params.gids, vec![1, 2, 3]);
            }
            _ => panic!("expected AuthSys"),
        }
    }
}

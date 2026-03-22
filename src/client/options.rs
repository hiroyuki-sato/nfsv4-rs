use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RpcCallOptions {
    pub timeout: Duration,
    pub xid: Option<u32>,
    pub retry_count: u32,
}

impl Default for RpcCallOptions {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            xid: None,
            retry_count: 0,
        }
    }
}

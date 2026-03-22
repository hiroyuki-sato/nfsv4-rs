#![allow(dead_code)]
#![allow(unused_imports)] // TODO: Remove this once all operations are implemented.

use crate::client::config::{AuthConfig, LowLevelClientConfig, NfsMinorVersion, ServerAddr};
use crate::client::options::RpcCallOptions;
use crate::client::session::SessionState;
use crate::error::Nfsv4Error;
use crate::nfsv4::compound::{Compound4Args, Compound4Res};
use crate::nfsv4::ops::{CreateSession4Args, CreateSession4Res, ExchangeId4Args, ExchangeId4Res};

/// Low-level NFSv4 client.
///
/// This client exposes protocol-level operations and COMPOUND sending.
/// It does not try to provide a file-oriented high-level API.
pub struct LowLevelClient {
    config: LowLevelClientConfig,
    session: SessionState,
    // TODO: replace with actual ONC RPC transport/client type
    // rpc: OncRpcClient,
}

impl LowLevelClient {
    /// Create a client instance from explicit config.
    pub async fn connect(config: LowLevelClientConfig) -> Result<Self, Nfsv4Error> {
        let _server = config.server.socket_addr();

        // TODO: establish TCP connection and create ONC RPC client
        // TODO: negotiate auth flavor

        Ok(Self {
            config,
            session: SessionState::default(),
        })
    }

    /// Returns the immutable config used by this client.
    pub fn config(&self) -> &LowLevelClientConfig {
        &self.config
    }

    /// Returns current session/bootstrap state.
    pub fn session(&self) -> &SessionState {
        &self.session
    }

    /// Send raw COMPOUND request.
    pub async fn compound(&mut self, args: Compound4Args) -> Result<Compound4Res, Nfsv4Error> {
        // TODO: encode COMPOUND4args into ONC RPC call
        // TODO: send request and decode COMPOUND4res
        let _ = args;
        Err(Nfsv4Error::InvalidData("compound() not implemented".into()))
    }

    /// Perform EXCHANGE_ID for NFSv4.1.
    pub async fn exchange_id(
        &mut self,
        args: ExchangeId4Args,
    ) -> Result<ExchangeId4Res, Nfsv4Error> {
        let _ = args;

        // TODO: send OP_EXCHANGE_ID inside COMPOUND
        // TODO: update self.session.clientid on success
        Err(Nfsv4Error::InvalidData(
            "exchange_id() not implemented".into(),
        ))
    }

    /// Perform CREATE_SESSION for NFSv4.1.
    pub async fn create_session(
        &mut self,
        args: CreateSession4Args,
    ) -> Result<CreateSession4Res, Nfsv4Error> {
        let _ = args;

        // TODO: send OP_CREATE_SESSION inside COMPOUND
        // TODO: update self.session.sessionid / sequenceid / slot state on success
        Err(Nfsv4Error::InvalidData(
            "create_session() not implemented".into(),
        ))
    }

    /// Convenience bootstrap for NFSv4.1.
    ///
    /// Intended to perform:
    /// 1. EXCHANGE_ID
    /// 2. CREATE_SESSION
    pub async fn bootstrap_v41(
        &mut self,
        exchange_id: ExchangeId4Args,
        create_session: CreateSession4Args,
    ) -> Result<(), Nfsv4Error> {
        let _exchange_res = self.exchange_id(exchange_id).await?;
        let _session_res = self.create_session(create_session).await?;
        Ok(())
    }

    /// Reset locally tracked session state.
    pub fn clear_session_state(&mut self) {
        self.session = SessionState::default();
    }
}

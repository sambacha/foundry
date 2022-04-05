mod config;

use crate::{
    eth::{backend::mem, miner::MiningMode, pool::Pool, EthApi},
    revm::{CfgEnv, TxEnv},
    service::NodeService,
};
pub use config::NodeConfig;
use foundry_evm::{revm, revm::BlockEnv};

use parking_lot::RwLock;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tokio::task::JoinHandle;

mod service;

/// axum RPC server implementations
pub mod server;

pub mod eth;

/// Creates the node and runs the server
///
/// Returns the [EthApi] that can be used to interact with the node and the [JoinHandle] of the
/// task.
///
/// # Example
///
/// ```rust
/// # use forge_node::NodeConfig;
/// # async fn spawn() {
/// let config = NodeConfig::default();
/// let(api, handle) = forge_node::spawn(config);
///
/// // use api
///
/// // wait forever
/// handle.await.unwrap();
/// # }
/// ```
pub fn spawn(config: NodeConfig) -> (EthApi, JoinHandle<hyper::Result<()>>) {
    // set everything up
    let NodeConfig {
        chain_id,
        gas_limit,
        gas_price: _,
        genesis_accounts: _,
        genesis_balance: _,
        accounts: _,
        automine,
        port,
        max_transactions,
    } = config;

    let env = revm::Env {
        cfg: CfgEnv { ..Default::default() },
        block: BlockEnv { gas_limit, ..Default::default() },
        tx: TxEnv { chain_id: Some(chain_id), ..Default::default() },
    };

    let pool = Arc::new(Pool::default());

    let mode = if let Some(automine) = automine {
        MiningMode::interval(automine)
    } else {
        let listener = pool.add_ready_listener();
        MiningMode::instant(max_transactions, listener)
    };

    // only memory based backend for now
    let backend = Arc::new(mem::Backend::empty(Arc::new(RwLock::new(env))));

    let api = EthApi::new(Arc::clone(&pool), Arc::clone(&backend), Default::default());

    let node_service = NodeService::new(pool, backend, mode);

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);

    let serve = server::serve(socket, api.clone());

    // spawn the server and the node service and poll as long as both are running
    let handle = tokio::task::spawn(async move {
        loop {
            tokio::select! {
                res = serve => {
                    return res
                },
                res = node_service => {
                     return res
                }
            }
        }
    });

    (api, handle)
}

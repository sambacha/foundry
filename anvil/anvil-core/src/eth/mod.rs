use crate::{
    eth::{call::CallRequest, filter::Filter, transaction::EthTransactionRequest},
    types::{EvmMineOptions, Forking, Index},
};
use ethers_core::{
    abi::ethereum_types::H64,
    types::{Address, BlockNumber, Bytes, TxHash, H256, U256},
};
use serde::{Deserialize, Deserializer};

pub mod block;
pub mod call;
pub mod filter;
pub mod receipt;
pub mod transaction;
pub mod trie;
pub mod utils;

/// Represents ethereum JSON-RPC API
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(tag = "method", content = "params")]
pub enum EthRequest {
    #[serde(rename = "eth_chainId")]
    EthChainId,

    #[serde(rename = "eth_gasPrice")]
    EthGasPrice,

    #[serde(rename = "eth_accounts")]
    EthAccounts,

    #[serde(rename = "eth_blockNumber")]
    EthBlockNumber,

    #[serde(rename = "eth_getBalance")]
    EthGetBalance(Address, Option<BlockNumber>),

    #[serde(rename = "eth_getStorageAt")]
    EthGetStorageAt(Address, U256, Option<BlockNumber>),

    #[serde(rename = "eth_getBlockByHash")]
    EthGetBlockByHash(H256, bool),

    #[serde(rename = "eth_getBlockByNumber")]
    EthGetBlockByNumber(BlockNumber, bool),

    #[serde(rename = "eth_getTransactionCount")]
    EthGetTransactionCount(Address, Option<BlockNumber>),

    #[serde(rename = "eth_getBlockTransactionCountByHash")]
    EthGetTransactionCountByHash(H256),

    #[serde(rename = "eth_getBlockTransactionCountByNumber")]
    EthGetTransactionCountByNumber(BlockNumber),

    #[serde(rename = "eth_getUncleCountByBlockHash")]
    EthGetUnclesCountByHash(H256),

    #[serde(rename = "eth_getUncleCountByBlockNumber")]
    EthGetUnclesCountByNumber(BlockNumber),

    #[serde(rename = "eth_getCode")]
    EthGetCodeAt(Address, Option<BlockNumber>),

    #[serde(rename = "eth_sendTransaction", with = "sequence")]
    EthSendTransaction(Box<EthTransactionRequest>),

    #[serde(rename = "eth_sendRawTransaction", with = "sequence")]
    EthSendRawTransaction(Bytes),

    #[serde(rename = "eth_call")]
    EthCall(CallRequest, #[serde(default)] Option<BlockNumber>),

    #[serde(rename = "eth_estimateGas")]
    EthEstimateGas(CallRequest, #[serde(default)] Option<BlockNumber>),

    #[serde(rename = "eth_getTransactionByHash", with = "sequence")]
    EthGetTransactionByHash(TxHash),

    #[serde(rename = "eth_getTransactionByBlockHashAndIndex")]
    EthGetTransactionByBlockHashAndIndex(TxHash, Index),

    #[serde(rename = "eth_getTransactionByBlockNumberAndIndex")]
    EthGetTransactionByBlockNumberAndIndex(BlockNumber, Index),

    #[serde(rename = "eth_getTransactionReceipt", with = "sequence")]
    EthGetTransactionReceipt(H256),

    #[serde(rename = "eth_getUncleByBlockHashAndIndex")]
    EthGetUncleByBlockHashAndIndex(H256, Index),

    #[serde(rename = "eth_getUncleByBlockNumberAndIndex")]
    EthGetUncleByBlockNumberAndIndex(BlockNumber, Index),

    #[serde(rename = "eth_getLogs")]
    EthGetLogs(Filter),

    #[serde(rename = "eth_getWork")]
    EthGetWork,

    #[serde(rename = "eth_submitWork")]
    EthSubmitWork(H64, H256, H256),

    #[serde(rename = "eth_submitHashrate")]
    EthSubmitHashRate(U256, H256),

    #[serde(rename = "eth_feeHistory")]
    EthFeeHistory(
        #[serde(deserialize_with = "deserialize_number")] U256,
        BlockNumber,
        #[serde(default)] Vec<f64>,
    ),

    /// non-standard endpoint for traces
    #[serde(rename = "debug_traceTransaction", with = "sequence")]
    DebugTraceTransaction(H256),

    // Custom endpoints, they're not extracted to a separate type out of serde convenience
    /// send transactions impersonating specific account and contract addresses.
    #[serde(
        rename = "forge_impersonateAccount",
        alias = "hardhat_impersonateAccount",
        with = "sequence"
    )]
    ImpersonateAccount(Address),
    /// Stops impersonating an account if previously set with `forge_impersonateAccount`
    #[serde(rename = "forge_stopImpersonatingAccount", alias = "hardhat_stopImpersonatingAccount")]
    StopImpersonatingAccount,
    /// Returns true if automatic mining is enabled, and false.
    #[serde(rename = "forge_getAutomine", alias = "hardhat_getAutomine")]
    GetAutoMine,
    /// Mines a series of blocks
    #[serde(rename = "forge_mine", alias = "hardhat_mine")]
    Mine(
        /// Number of blocks to mine, if not set `1` block is mined
        #[serde(default, deserialize_with = "deserialize_number_opt")]
        Option<U256>,
        /// The time interval between each block in seconds, defaults to `1` seconds
        /// The interval is applied only to blocks mined in the given method invocation, not to
        /// blocks mined afterwards. Set this to `0` to instantly mine _all_ blocks
        #[serde(default, deserialize_with = "deserialize_number_opt")]
        Option<U256>,
    ),

    /// Enables or disables, based on the single boolean argument, the automatic mining of new
    /// blocks with each new transaction submitted to the network.
    #[serde(rename = "evm_setAutomine", with = "sequence")]
    SetAutomine(bool),

    /// Sets the mining behavior to interval with the given interval (seconds)
    #[serde(rename = "evm_setIntervalMining", with = "sequence")]
    SetIntervalMining(u64),

    /// Removes transactions from the pool
    #[serde(
        rename = "forge_dropTransaction",
        alias = "hardhat_dropTransaction",
        with = "sequence"
    )]
    DropTransaction(H256),

    /// Reset the fork to a fresh forked state, and optionally update the fork config
    #[serde(rename = "forge_reset", alias = "hardhat_reset", with = "sequence")]
    Reset(#[serde(default)] Forking),

    /// Modifies the balance of an account.
    #[serde(rename = "forge_setBalance", alias = "hardhat_setBalance")]
    SetBalance(Address, #[serde(deserialize_with = "deserialize_number")] U256),

    /// Sets the code of a contract
    #[serde(rename = "forge_setCode", alias = "hardhat_setCode")]
    SetCode(Address, Bytes),

    /// Sets the nonce of an address
    #[serde(rename = "forge_setNonce", alias = "hardhat_setNonce")]
    SetNonce(Address, #[serde(deserialize_with = "deserialize_number")] U256),

    /// Writes a single slot of the account's storage
    #[serde(rename = "forge_setStorageAt", alias = "hardhat_setStorageAt")]
    SetStorageAt(
        Address,
        /// slot
        U256,
        /// value
        U256,
    ),

    /// Sets the coinbase address
    #[serde(rename = "forge_setCoinbase", alias = "hardhat_setCoinbase", with = "sequence")]
    SetCoinbase(Address),

    /// Enable or disable logging
    #[serde(
        rename = "forge_setLoggingEnabled",
        alias = "hardhat_setLoggingEnabled",
        with = "sequence"
    )]
    SetLogging(bool),

    /// Set the minimum gas price for the node
    #[serde(rename = "forge_setMinGasPrice", alias = "hardhat_setMinGasPrice", with = "sequence")]
    SetMinGasPrice(#[serde(deserialize_with = "deserialize_number")] U256),

    /// Sets the base fee of the next block
    #[serde(
        rename = "forge_setNextBlockBaseFeePerGas",
        alias = "hardhat_setNextBlockBaseFeePerGas",
        with = "sequence"
    )]
    SetNextBlockBaseFeePerGas(#[serde(deserialize_with = "deserialize_number")] U256),

    // Ganache compatible calls
    /// Snapshot the state of the blockchain at the current block.
    #[serde(rename = "evm_snapshot")]
    EvmSnapshot,

    /// Revert the state of the blockchain to a previous snapshot.
    /// Takes a single parameter, which is the snapshot id to revert to.
    #[serde(rename = "evm_revert", with = "sequence")]
    EvmRevert(#[serde(deserialize_with = "deserialize_number")] U256),

    /// Jump forward in time by the given amount of time, in seconds.
    #[serde(rename = "evm_increaseTime", with = "sequence")]
    EvmIncreaseTime(#[serde(deserialize_with = "deserialize_number")] U256),

    /// Similar to `evm_increaseTime` but takes the exact timestamp that you want in the next block
    #[serde(rename = "evm_setNextBlockTimestamp", with = "sequence")]
    EvmSetNextBlockTimeStamp(u64),

    /// Mine a single block
    #[serde(rename = "evm_mine", with = "sequence")]
    EvmMine(EvmMineOptions),
}

/// Represents a non-standard forge JSON-RPC API, compatible with other dev nodes, hardhat, ganache

fn deserialize_number<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Numeric {
        U256(U256),
        Num(u64),
    }

    let num = match Numeric::deserialize(deserializer)? {
        Numeric::U256(n) => n,
        Numeric::Num(n) => U256::from(n),
    };

    Ok(num)
}

fn deserialize_number_opt<'de, D>(deserializer: D) -> Result<Option<U256>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Numeric {
        U256(U256),
        Num(u64),
    }

    let num = match Option::<Numeric>::deserialize(deserializer)? {
        Some(Numeric::U256(n)) => Some(n),
        Some(Numeric::Num(n)) => Some(U256::from(n)),
        _ => None,
    };

    Ok(num)
}

#[allow(unused)]
mod sequence {
    use serde::{
        de::DeserializeOwned, ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer,
    };

    #[allow(unused)]
    pub fn serialize<S, T>(val: &T, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        let mut seq = s.serialize_seq(Some(1))?;
        seq.serialize_element(val)?;
        seq.end()
    }

    pub fn deserialize<'de, T, D>(d: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: DeserializeOwned,
    {
        let mut seq = Vec::<T>::deserialize(d)?;
        if seq.len() != 1 {
            return Err(serde::de::Error::custom(format!(
                "expected params sequence with length 1 but got {}",
                seq.len()
            )))
        }
        Ok(seq.remove(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_impersonate_account() {
        let s = r#"{"method": "forge_impersonateAccount", "params": ["0xd84de507f3fada7df80908082d3239466db55a71"]}"#;
        let value: serde_json::Value = serde_json::from_str(s).unwrap();
        let _req = serde_json::from_value::<EthRequest>(value).unwrap();
    }

    #[test]
    fn test_custom_stop_impersonate_account() {
        let s = r#"{"method": "forge_stopImpersonatingAccount"}"#;
        let value: serde_json::Value = serde_json::from_str(s).unwrap();
        let _req = serde_json::from_value::<EthRequest>(value).unwrap();
    }

    #[test]
    fn test_custom_get_automine() {
        let s = r#"{"method": "forge_getAutomine"}"#;
        let value: serde_json::Value = serde_json::from_str(s).unwrap();
        let _req = serde_json::from_value::<EthRequest>(value).unwrap();
    }

    #[test]
    fn test_custom_mine() {
        let s = r#"{"method": "forge_mine", "params": []}"#;
        let value: serde_json::Value = serde_json::from_str(s).unwrap();
        let _req = serde_json::from_value::<EthRequest>(value).unwrap();
        let s =
            r#"{"method": "forge_mine", "params": ["0xd84de507f3fada7df80908082d3239466db55a71"]}"#;
        let value: serde_json::Value = serde_json::from_str(s).unwrap();
        let _req = serde_json::from_value::<EthRequest>(value).unwrap();
        let s = r#"{"method": "forge_mine", "params": ["0xd84de507f3fada7df80908082d3239466db55a71", "0xd84de507f3fada7df80908082d3239466db55a71"]}"#;
        let value: serde_json::Value = serde_json::from_str(s).unwrap();
        let _req = serde_json::from_value::<EthRequest>(value).unwrap();
    }

    #[test]
    fn test_custom_auto_mine() {
        let s = r#"{"method": "evm_setAutomine", "params": [false]}"#;
        let value: serde_json::Value = serde_json::from_str(s).unwrap();
        let _req = serde_json::from_value::<EthRequest>(value).unwrap();
    }

    #[test]
    fn test_custom_interval_mining() {
        let s = r#"{"method": "evm_setIntervalMining", "params": [100]}"#;
        let value: serde_json::Value = serde_json::from_str(s).unwrap();
        let _req = serde_json::from_value::<EthRequest>(value).unwrap();
    }

    #[test]
    fn test_custom_drop_tx() {
        let s = r#"{"method": "forge_dropTransaction", "params": ["0x4a3b0fce2cb9707b0baa68640cf2fe858c8bb4121b2a8cb904ff369d38a560ff"]}"#;
        let value: serde_json::Value = serde_json::from_str(s).unwrap();
        let _req = serde_json::from_value::<EthRequest>(value).unwrap();
    }

    #[test]
    fn test_custom_reset() {
        let s = r#"{"method": "forge_reset", "params": [ {
            "forking" : {
                "jsonRpcUrl": "https://eth-mainnet.alchemyapi.io/v2/<key>",
                "blockNumber": 11095000
            }
        }]}"#;
        let value: serde_json::Value = serde_json::from_str(s).unwrap();
        let _req = serde_json::from_value::<EthRequest>(value).unwrap();
    }

    #[test]
    fn test_custom_set_balance() {
        let s = r#"{"method": "forge_setBalance", "params": ["0xd84de507f3fada7df80908082d3239466db55a71", "0x0"]}"#;
        let value: serde_json::Value = serde_json::from_str(s).unwrap();
        let _req = serde_json::from_value::<EthRequest>(value).unwrap();
    }

    #[test]
    fn test_custom_set_code() {
        let s = r#"{"method": "forge_setCode", "params": ["0xd84de507f3fada7df80908082d3239466db55a71", "0x0123456789abcdef"]}"#;
        let value: serde_json::Value = serde_json::from_str(s).unwrap();
        let _req = serde_json::from_value::<EthRequest>(value).unwrap();
    }

    #[test]
    fn test_custom_set_nonce() {
        let s = r#"{"method": "forge_setNonce", "params": ["0xd84de507f3fada7df80908082d3239466db55a71", "0x0"]}"#;
        let value: serde_json::Value = serde_json::from_str(s).unwrap();
        let _req = serde_json::from_value::<EthRequest>(value).unwrap();
    }

    #[test]
    fn test_serde_custom_set_storage_at() {
        let s = r#"{"method": "forge_setStorageAt", "params": ["0x295a70b2de5e3953354a6a8344e616ed314d7251", "0x0", "0x00"]}"#;
        let value: serde_json::Value = serde_json::from_str(s).unwrap();
        let _req = serde_json::from_value::<EthRequest>(value).unwrap();
    }

    #[test]
    fn test_serde_custom_coinbase() {
        let s = r#"{"method": "forge_setCoinbase", "params": ["0x295a70b2de5e3953354a6a8344e616ed314d7251"]}"#;
        let value: serde_json::Value = serde_json::from_str(s).unwrap();
        let _req = serde_json::from_value::<EthRequest>(value).unwrap();
    }

    #[test]
    fn test_serde_custom_logging() {
        let s = r#"{"method": "forge_setLoggingEnabled", "params": [false]}"#;
        let value: serde_json::Value = serde_json::from_str(s).unwrap();
        let _req = serde_json::from_value::<EthRequest>(value).unwrap();
    }

    #[test]
    fn test_serde_custom_min_gas_price() {
        let s = r#"{"method": "forge_setMinGasPrice", "params": ["0x0"]}"#;
        let value: serde_json::Value = serde_json::from_str(s).unwrap();
        let _req = serde_json::from_value::<EthRequest>(value).unwrap();
    }

    #[test]
    fn test_serde_custom_next_block_base_fee() {
        let s = r#"{"method": "forge_setNextBlockBaseFeePerGas", "params": ["0x0"]}"#;
        let value: serde_json::Value = serde_json::from_str(s).unwrap();
        let _req = serde_json::from_value::<EthRequest>(value).unwrap();
    }

    #[test]
    fn test_serde_custom_snapshot() {
        let s = r#"{"method": "evm_snapshot"}"#;
        let value: serde_json::Value = serde_json::from_str(s).unwrap();
        let _req = serde_json::from_value::<EthRequest>(value).unwrap();
    }

    #[test]
    fn test_serde_custom_revert() {
        let s = r#"{"method": "evm_revert", "params": ["0x0"]}"#;
        let value: serde_json::Value = serde_json::from_str(s).unwrap();
        let _req = serde_json::from_value::<EthRequest>(value).unwrap();
    }

    #[test]
    fn test_serde_custom_increase_time() {
        let s = r#"{"method": "evm_increaseTime", "params": ["0x0"]}"#;
        let value: serde_json::Value = serde_json::from_str(s).unwrap();
        let _req = serde_json::from_value::<EthRequest>(value).unwrap();
    }

    #[test]
    fn test_serde_custom_next_timestamp() {
        let s = r#"{"method": "evm_setNextBlockTimestamp", "params": [100]}"#;
        let value: serde_json::Value = serde_json::from_str(s).unwrap();
        let _req = serde_json::from_value::<EthRequest>(value).unwrap();
    }

    #[test]
    fn test_serde_custom_evm_mine() {
        let s = r#"{"method": "evm_mine", "params": [100]}"#;
        let value: serde_json::Value = serde_json::from_str(s).unwrap();
        let _req = serde_json::from_value::<EthRequest>(value).unwrap();
        let s = r#"{"method": "evm_mine", "params": [{
            "timestamp": 100,
            "blocks": 100
        }]}"#;
        let value: serde_json::Value = serde_json::from_str(s).unwrap();
        let _req = serde_json::from_value::<EthRequest>(value).unwrap();
    }

    #[test]
    fn test_serde_eth_storage() {
        let s = r#"{"method": "eth_getStorageAt", "params": ["0x295a70b2de5e3953354a6a8344e616ed314d7251", "0x0", "latest"]}"#;
        let value: serde_json::Value = serde_json::from_str(s).unwrap();
        let _req = serde_json::from_value::<EthRequest>(value).unwrap();
    }

    #[test]
    fn test_eth_call() {
        let req = r#"{"data":"0xcfae3217","from":"0xd84de507f3fada7df80908082d3239466db55a71","to":"0xcbe828fdc46e3b1c351ec90b1a5e7d9742c0398d"}"#;
        let _req = serde_json::from_str::<CallRequest>(req).unwrap();

        let s = r#"{"method": "eth_call", "params":  [{"data":"0xcfae3217","from":"0xd84de507f3fada7df80908082d3239466db55a71","to":"0xcbe828fdc46e3b1c351ec90b1a5e7d9742c0398d"},"latest"]}"#;
        let _req = serde_json::from_str::<EthRequest>(s).unwrap();

        let s = r#"{"method": "eth_call", "params":  [{"data":"0xcfae3217","from":"0xd84de507f3fada7df80908082d3239466db55a71","to":"0xcbe828fdc46e3b1c351ec90b1a5e7d9742c0398d"}]}"#;
        let _req = serde_json::from_str::<EthRequest>(s).unwrap();
    }

    #[test]
    fn test_serde_eth_balance() {
        let s = r#"{"method": "eth_getBalance", "params": ["0x295a70b2de5e3953354a6a8344e616ed314d7251", "latest"]}"#;
        let value: serde_json::Value = serde_json::from_str(s).unwrap();

        let _req = serde_json::from_value::<EthRequest>(value).unwrap();
    }
}

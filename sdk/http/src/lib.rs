mod gateway_http_proxy;

pub use gateway_http_proxy::GatewayHttpProxy;

pub use dharitri_sdk as core;

pub const MAINNET_GATEWAY: &str = "https://gateway.dharitri.com";
pub const TESTNET_GATEWAY: &str = "https://testnet-gateway.dharitri.com";
pub const DEVNET_GATEWAY: &str = "https://devnet-gateway.dharitri.com";
pub const CHAIN_SIMULATOR_GATEWAY: &str = "http://localhost:8085";

// MetachainShardId will be used to identify a shard ID as metachain
pub const METACHAIN_SHARD_ID: u32 = 0xFFFFFFFF;

pub const DEFAULT_USE_CHAIN_SIMULATOR: bool = false;

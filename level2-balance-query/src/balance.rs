use alloy::primitives::{Address, U256};
use alloy::providers::{Provider, ProviderBuilder};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // RPC URL,连接 Arbitrum Sepolia
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;

    // 构建 HTTP Provider
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    // 解析地址字符串
    let address_str = "0x94a517e9959eed4A8319f73166cb7725ae4cC8f0";
    let address: Address = address_str.parse()?;

    // 查询余额(wei)
    let balance_wei: U256 = provider.get_balance(address).await?;

    // wei -> ETH
    let balance_eth = wei_to_eth(balance_wei);

    println!("Address: {}", address_str);
    println!("Balance: {} ETH", balance_eth);

    Ok(())
}

/// 将 wei 转换为 ETH(可读浮点)
fn wei_to_eth(wei: U256) -> f64 {
    let wei_f64 = wei.to_string().parse::<f64>().unwrap_or(0.0);
    wei_f64 / 1e18
}

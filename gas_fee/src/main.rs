use alloy::providers::{Provider, ProviderBuilder};
use eyre::Result;

/// Arbitrum Sepolia 测试网 RPC URL
const ARBITRUM_SEPOLIA_RPC: &str = "https://arbitrum-sepolia-rpc.publicnode.com";
/// 基础 ETH 转账的 Gas 限额（行业通用值：21000）
const BASE_TRANSFER_GAS_LIMIT: u128 = 21000;

/// 获取当前 Gas 价格并计算预估转账费用
async fn estimate_transfer_gas_fee() -> Result<()> {
    // 连接到 Arbitrum Sepolia 测试网
    let provider = ProviderBuilder::new()
        .connect_http(ARBITRUM_SEPOLIA_RPC.parse().unwrap());

    // 获取当前的 Gas 价格
    let gas_price = provider.get_gas_price().await?;
    
    println!("=== Arbitrum Sepolia Gas 信息 ===");
    println!("当前 Gas 价格: {} Wei", gas_price);
    println!("当前 Gas 价格: {} Gwei", gas_price as f64 / 1e9);
    
    // 计算基础转账的预估 Gas 费用
    // Gas 费 = 当前 Gas 价格 × Gas 限额
    let estimated_gas_fee = gas_price * BASE_TRANSFER_GAS_LIMIT;
    
    println!("\n=== 基础转账费用估算 ===");
    println!("Gas 限额: {} Gas", BASE_TRANSFER_GAS_LIMIT);
    println!("预估 Gas 费: {} Wei", estimated_gas_fee);
    println!("预估 Gas 费: {} Gwei", estimated_gas_fee as f64 / 1e9);
    println!("预估 Gas 费: {} ETH", estimated_gas_fee as f64 / 1e18);
    
    println!("\n计算公式: Gas 费 = Gas 价格 × Gas 限额");
    println!("         {} Wei = {} Wei/Gas × {} Gas", 
             estimated_gas_fee, gas_price, BASE_TRANSFER_GAS_LIMIT);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("正在获取 Arbitrum Sepolia 测试网 Gas 信息...\n");
    
    estimate_transfer_gas_fee().await?;
    
    Ok(())
}

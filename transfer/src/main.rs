use alloy::network::{EthereumWallet, TransactionBuilder};
use alloy::primitives::{Address, U256, utils::parse_ether};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::TransactionRequest;
use alloy::signers::local::PrivateKeySigner;
use eyre::{Result, eyre};
use std::env;

/// Arbitrum Sepolia 测试网 RPC URL
const ARBITRUM_SEPOLIA_RPC: &str = "https://arbitrum-sepolia-rpc.publicnode.com";

/// 基础 ETH 转账的 Gas 限额
const BASE_TRANSFER_GAS_LIMIT: u64 = 30000;

/// 从环境变量加载私钥并创建签名器
fn load_signer_from_env() -> Result<PrivateKeySigner> {
    // 从环境变量读取私钥
    let private_key = env::var("PRIVATE_KEY")
        .map_err(|_| eyre!("未找到 PRIVATE_KEY 环境变量。请设置: export PRIVATE_KEY=your_private_key"))?;
    
    // 创建签名器
    let signer: PrivateKeySigner = private_key.parse()
        .map_err(|_| eyre!("私钥格式无效。请确保私钥格式正确（0x开头的64位十六进制字符串）"))?;
    
    Ok(signer)
}

/// 验证地址格式
fn validate_address(address_str: &str, name: &str) -> Result<Address> {
    address_str.parse::<Address>()
        .map_err(|_| eyre!("{} 地址格式无效: {}", name, address_str))
}

/// 查询地址余额
async fn check_balance(provider: &impl Provider, address: Address) -> Result<U256> {
    let balance = provider.get_balance(address).await?;
    Ok(balance)
}

/// 执行 ETH 转账
async fn transfer_eth(
    from: Address,
    to: Address,
    amount_eth: &str,
) -> Result<()> {
    println!("=== Arbitrum Sepolia ETH 转账 ===\n");
    
    // 1. 验证地址
    println!("📍 转账地址:");
    println!("   发送方 (From): {}", from);
    println!("   接收方 (To):   {}", to);
    
    // 2. 连接到 Arbitrum Sepolia 测试网
    let signer = load_signer_from_env()?;
    let wallet = EthereumWallet::from(signer);
    
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect_http(ARBITRUM_SEPOLIA_RPC.parse()?);
    
    // 3. 检查发送方余额
    println!("\n 余额检查:");
    let balance = check_balance(&provider, from).await?;
    let balance_eth = balance.to::<u128>() as f64 / 1e18;
    println!("   发送方余额: {} ETH ({} Wei)", balance_eth, balance);
    
    // 解析转账金额
    let amount = parse_ether(amount_eth)?;
    let amount_eth_f64 = amount.to::<u128>() as f64 / 1e18;
    println!("   转账金额:   {} ETH ({} Wei)", amount_eth_f64, amount);
    
    // 4. 获取当前 Gas 价格并估算费用
    println!("\n Gas 费用估算:");
    let gas_price = provider.get_gas_price().await?;
    let estimated_gas_fee = U256::from(gas_price) * U256::from(BASE_TRANSFER_GAS_LIMIT);
    
    println!("   当前 Gas 价格: {} Gwei", gas_price as f64 / 1e9);
    println!("   Gas 限额:      {} Gas", BASE_TRANSFER_GAS_LIMIT);
    println!("   预估 Gas 费:   {} ETH ({} Wei)", 
             estimated_gas_fee.to::<u128>() as f64 / 1e18, estimated_gas_fee);
    
    // 5. 检查余额是否足够（转账金额 + Gas 费）
    let total_needed = amount + estimated_gas_fee;
    if balance < total_needed {
        return Err(eyre!(
            "余额不足！需要 {} ETH(转账 {} + Gas 费 {}），但只有 {} ETH",
            total_needed.to::<u128>() as f64 / 1e18,
            amount_eth_f64,
            estimated_gas_fee.to::<u128>() as f64 / 1e18,
            balance_eth
        ));
    }
    
    println!("  余额充足");
    
    // 6. 构建交易
    println!("\n 构建交易...");
    let tx = TransactionRequest::default()
        .with_to(to)
        .with_value(amount)
        .with_gas_limit(BASE_TRANSFER_GAS_LIMIT);
    
    // 7. 发送交易
    println!("  签名并发送交易...");
    let pending_tx = provider.send_transaction(tx).await?;
    let tx_hash = pending_tx.tx_hash();
    
    println!("\n 交易已发送!");
    println!("   交易哈希: {}", tx_hash);
    println!("   浏览器查看: https://sepolia.arbiscan.io/tx/{}", tx_hash);
    
    // 8. 等待交易确认
    println!("\n 等待交易确认...");
    let receipt = pending_tx.get_receipt().await?;
    
    println!("\n 交易成功确认!");
    println!("   区块号:     {}", receipt.block_number.unwrap_or_default());
    println!("   Gas 使用:   {} Gas", receipt.gas_used);
    let actual_gas_fee = receipt.gas_used as u128 * receipt.effective_gas_price;
    println!("   实际 Gas 费: {} ETH", 
             actual_gas_fee as f64 / 1e18);
    println!("   交易状态:   {}", if receipt.status() { "成功 " } else { "失败 " });
    
    // 9. 查询转账后的余额
    println!("\n 转账后余额:");
    let new_balance = check_balance(&provider, from).await?;
    let new_balance_eth = new_balance.to::<u128>() as f64 / 1e18;
    println!("   发送方余额: {} ETH", new_balance_eth);
    
    let to_balance = check_balance(&provider, to).await?;
    let to_balance_eth = to_balance.to::<u128>() as f64 / 1e18;
    println!("   接收方余额: {} ETH", to_balance_eth);
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // 加载 .env 文件（如果存在）
    dotenv::dotenv().ok();
    
    // 从环境变量或命令行参数获取转账信息
    let args: Vec<String> = env::args().collect();
    
    // 检查是否提供了接收地址和转账金额
    if args.len() < 3 {
        println!("使用方法:");
        println!("  cargo run <接收地址> <转账金额ETH>");
        println!("\n示例:");
        println!("  cargo run 0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb 0.001");
        println!("\n环境变量配置:");
        println!("  export PRIVATE_KEY=your_private_key_here");
        println!("  或者在项目根目录创建 .env 文件，内容:");
        println!("  PRIVATE_KEY=your_private_key_here");
        return Ok(());
    }
    
    let to_address_str = &args[1];
    let amount = &args[2];
    
    // 验证接收地址
    let to_address = validate_address(to_address_str, "接收方")?;
    
    // 从环境变量加载私钥并获取发送方地址
    let signer = load_signer_from_env()?;
    let from_address = signer.address();
    
    // 执行转账
    transfer_eth(from_address, to_address, amount).await?;
    
    Ok(())
}

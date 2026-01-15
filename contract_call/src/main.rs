use alloy::{
    providers::ProviderBuilder,
    sol,
    primitives::Address,
};
use std::str::FromStr;

// 使用 sol! 宏定义合约 ABI
// ERC20 标准合约，定义常用的只读方法
sol! {
    #[sol(rpc)]
    contract IERC20 {
        // 查询代币名称
        function name() external view returns (string memory);
        
        // 查询代币符号
        function symbol() external view returns (string memory);
        
        // 查询小数位数
        function decimals() external view returns (uint8);
        
        // 查询总供应量
        function totalSupply() external view returns (uint256);
        
        // 查询某地址的余额
        function balanceOf(address account) external view returns (uint256);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("开始与 Arbitrum 测试网合约交互...\n");

    // 设置 RPC 端点（Arbitrum Sepolia 测试网）
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";
    
    // 填写合约地址
    let contract_address = "0x2bF2A9E3A07B9f75fC1b36D56Efd6999b3AF7951"; // ERC20 合约地址
    
    println!("合约地址: {}", contract_address);
    println!("RPC 端点: {}\n", rpc_url);

    // 创建 provider
    let provider = ProviderBuilder::new()
        .connect_http(rpc_url.parse()?);

    // 解析合约地址
    let address = Address::from_str(contract_address)?;
    
    // 创建合约实例
    let contract = IERC20::new(address, &provider);

    println!("开始读取合约信息...\n");

    // 调用只读方法
    
    // 1. 查询代币名称
    match contract.name().call().await {
        Ok(name_result) => {
            println!("代币名称 (name): {}", name_result);
        }
        Err(e) => {
            println!("获取名称失败: {}", e);
        }
    }

    // 2. 查询代币符号
    match contract.symbol().call().await {
        Ok(symbol_result) => {
            println!("代币符号 (symbol): {}", symbol_result);
        }
        Err(e) => {
            println!("获取符号失败: {}", e);
        }
    }

    // 3. 查询小数位数
    match contract.decimals().call().await {
        Ok(decimals_result) => {
            println!("小数位数 (decimals): {}", decimals_result);
        }
        Err(e) => {
            println!("获取小数位数失败: {}", e);
        }
    }

    // 4. 查询总供应量
    match contract.totalSupply().call().await {
        Ok(supply_result) => {
            println!("总供应量 (totalSupply): {}", supply_result);
        }
        Err(e) => {
            println!("获取总供应量失败: {}", e);
        }
    }

    // 5. 查询特定地址的余额
    let query_address = Address::from_str("0x0000000000000000000000000000000000000000")?;
    match contract.balanceOf(query_address).call().await {
        Ok(balance_result) => {
            println!("地址余额 (balanceOf): {}", balance_result);
        }
        Err(e) => {
            println!("获取余额失败: {}", e);
        }
    }

    println!("\n合约交互完成！");

    Ok(())
}


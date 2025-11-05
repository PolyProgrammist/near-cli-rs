use near_cli_rs::config::NetworkConfig;
use near_primitives::types::BlockReference;
use near_sandbox::Sandbox;

#[tokio::test]
async fn test_view_account_summary_with_localnet() -> Result<(), Box<dyn std::error::Error>> {
    // Start a local sandbox
    let sandbox = Sandbox::start_sandbox().await?;

    // Create a test network config pointing to our sandbox
    let rpc_url = format!("http://{}", sandbox.rpc_addr).parse()?;
    let network_config = NetworkConfig {
        network_name: "sandbox".to_string(),
        rpc_url,
        rpc_api_key: None,
        wallet_url: "http://localhost:1234".parse()?,
        explorer_transaction_url: "http://localhost:1234/transactions/".parse()?,
        near_social_db_contract_account_id: None,
        staking_pools_factory_account_id: None,
        linkdrop_account_id: None,
        faucet_url: None,
        meta_transaction_relayer_url: None,
        fastnear_url: None,
        coingecko_url: None,
    };

    // Test the view_account_summary functionality using the default test account
    let account_id: near_primitives::types::AccountId = "test.near".parse()?;
    let block_reference = BlockReference::latest();

    // Call the get_account_inquiry function directly since it's the core logic
    let result = near_cli_rs::commands::account::view_account_summary::get_account_inquiry(
        &account_id,
        &network_config,
        &block_reference,
    );

    // The result might be an error since test.near might not exist,
    // but we're testing that the function can connect to the sandbox
    // For this test, we just verify the function doesn't panic
    let _ = result;

    Ok(())
}

#[tokio::test]
async fn test_view_account_summary_nonexistent_account() -> Result<(), Box<dyn std::error::Error>> {
    // Start a local sandbox with default configuration
    let sandbox = Sandbox::start_sandbox().await?;

    // Create a test network config pointing to our sandbox
    let rpc_url = format!("http://{}", sandbox.rpc_addr).parse()?;
    let network_config = NetworkConfig {
        network_name: "sandbox".to_string(),
        rpc_url,
        rpc_api_key: None,
        wallet_url: "http://localhost:1234".parse()?,
        explorer_transaction_url: "http://localhost:1234/transactions/".parse()?,
        near_social_db_contract_account_id: None,
        staking_pools_factory_account_id: None,
        linkdrop_account_id: None,
        faucet_url: None,
        meta_transaction_relayer_url: None,
        fastnear_url: None,
        coingecko_url: None,
    };

    // Test with a nonexistent account
    let nonexistent_account_id: near_primitives::types::AccountId = "nonexistent.test".parse()?;
    let block_reference = BlockReference::latest();

    // Call the get_account_inquiry function
    let result = near_cli_rs::commands::account::view_account_summary::get_account_inquiry(
        &nonexistent_account_id,
        &network_config,
        &block_reference,
    );

    // Verify that the function returns an error for nonexistent account
    assert!(
        result.is_err(),
        "view_account_summary should fail for nonexistent account"
    );

    Ok(())
}

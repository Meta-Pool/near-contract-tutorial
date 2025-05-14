use near_sdk::{AccountId, NearToken};
use serde_json::json;

#[tokio::test]
async fn test_contract_is_operational() -> Result<(), Box<dyn std::error::Error>> {
    let contract_wasm = near_workspaces::compile_project("./").await?;

    test_basics_on(&contract_wasm).await?;
    Ok(())
}

async fn test_basics_on(contract_wasm: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract = sandbox.dev_deploy(contract_wasm).await?;

    let user_account = sandbox.dev_create_account().await?;
    let user_account_2 = sandbox.dev_create_account().await?;

    let outcome_initalize = user_account
        .call(contract.id(), "new")
        .args_json(json!({"initial_owner": user_account.id()}))
        .transact()
        .await
        .unwrap();
    assert!(outcome_initalize.is_success());

    let outcome = user_account
        .call(contract.id(), "set_owner")
        .args_json(json!({"owner": user_account_2.id()}))
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await
        .unwrap();
    assert!(outcome.is_success());

    let user_message_outcome = contract.view("get_owner").args_json(json!({})).await?;
    assert_eq!(
        &user_message_outcome.json::<AccountId>()?,
        user_account_2.id()
    );

    Ok(())
}

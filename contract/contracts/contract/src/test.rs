#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::{Address as _},
    token, Address, Env, String,
};

fn setup_test() -> (Env, CarbonMarketplaceClient<'static>, Address, Address, token::Client<'static>, token::StellarAssetClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();

    // 1. Create a generic token (e.g., USDC or XLM) for payment
    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_client = token::Client::new(&env, &token_contract.address());
    let token_admin_client = token::StellarAssetClient::new(&env, &token_contract.address());

    // 2. Deploy Carbon Marketplace
    let contract_id = env.register(CarbonMarketplace, ());
    let market_client = CarbonMarketplaceClient::new(&env, &contract_id);

    // 3. Initialize Contract with Admin and Token
    let admin = Address::generate(&env);
    market_client.init(&admin, &token_client.address);

    (env, market_client, admin, token_admin, token_client, token_admin_client)
}

#[test]
fn test_fractional_purchase_and_escrow() {
    let (env, market, admin, _, token_client, token_admin) = setup_test();

    let creator = Address::generate(&env);
    let buyer = Address::generate(&env);

    // Mint some test tokens to buyer
    token_admin.mint(&buyer, &10_000);
    assert_eq!(token_client.balance(&buyer), 10_000);

    // 1. Creator creates credit of 100 tons
    let credit_id = market.create_credit(
        &creator,
        &String::from_str(&env, "Amazon Reforestation"),
        &100,
    );

    // 2. Admin verifies credit
    market.verify_credit(&admin, &credit_id, &VerificationStatus::Verified);

    // 3. Creator lists credit at 50 USDC per ton
    let price_per_ton = 50;
    market.list_credit(&creator, &credit_id, &price_per_ton);

    // 4. Buyer purchases 10 tons of credit (10 * 50 = 500 escrowed)
    let amount_to_buy = 10;
    let purchase_id = market.buy_credit(&buyer, &credit_id, &amount_to_buy);
    
    // Check escrow holds funds (500)
    assert_eq!(token_client.balance(&buyer), 9_500);
    assert_eq!(token_client.balance(&market.address), 500); // Contract holds the escrow
    assert_eq!(token_client.balance(&creator), 0);

    // Parent credit should have 90 left
    let parent_credit = market.get_credit(&credit_id);
    assert_eq!(parent_credit.carbon_amount, 90);

    // 5. Buyer confirms delivery (funds go to creator, new fractional ownership token generated)
    let fractional_credit_id = market.confirm_delivery(&buyer, &purchase_id);

    // Check final balances
    assert_eq!(token_client.balance(&market.address), 0);
    assert_eq!(token_client.balance(&creator), 500); // Creator got paid 500

    // Check ownership of the newly minted fractional credit
    let new_credit = market.get_credit(&fractional_credit_id);
    assert_eq!(new_credit.owner_address, buyer);
    assert_eq!(new_credit.carbon_amount, 10);
    assert_eq!(new_credit.project_name, String::from_str(&env, "Amazon Reforestation"));
}

#[test]
#[should_panic(expected = "Credit not verified")]
fn test_cannot_list_unverified() {
    let (env, market, _admin, _, _, _) = setup_test();
    let creator = Address::generate(&env);

    let credit_id = market.create_credit(
        &creator,
        &String::from_str(&env, "Fake Project"),
        &100,
    );

    market.list_credit(&creator, &credit_id, &50); // Should panic
}

#[test]
#[should_panic(expected = "Deadline has not expired yet")]
fn test_cannot_cancel_before_deadline() {
    let (env, market, admin, _, token_client, token_admin) = setup_test();
    let creator = Address::generate(&env);
    let buyer = Address::generate(&env);
    
    token_admin.mint(&buyer, &10_000);
    let credit_id = market.create_credit(&creator, &String::from_str(&env, "Wind Project"), &100);
    market.verify_credit(&admin, &credit_id, &VerificationStatus::Verified);
    market.list_credit(&creator, &credit_id, &50);

    let purchase_id = market.buy_credit(&buyer, &credit_id, &10); // costs 500
    
    // Attempting to cancel instantly should PANIC because time-lock is 7 days
    market.cancel_purchase(&creator, &purchase_id);
}

#[test]
fn test_admin_dispute_resolution() {
    let (env, market, admin, _, token_client, token_admin) = setup_test();
    let creator = Address::generate(&env);
    let buyer = Address::generate(&env);
    
    token_admin.mint(&buyer, &10_000);
    let credit_id = market.create_credit(&creator, &String::from_str(&env, "Solar Project"), &100);
    market.verify_credit(&admin, &credit_id, &VerificationStatus::Verified);
    market.list_credit(&creator, &credit_id, &50);

    // Buy 20 tons = 1000 cost
    let purchase_id = market.buy_credit(&buyer, &credit_id, &20); 
    
    // Buyer marks as disputed
    market.mark_disputed(&buyer, &purchase_id);

    // Admin examines and resolves dispute, refunding buyer (true)
    market.resolve_dispute(&admin, &purchase_id, &true);

    // Escrow completely refunded
    assert_eq!(token_client.balance(&buyer), 10_000);
    assert_eq!(token_client.balance(&market.address), 0);
    assert_eq!(token_client.balance(&creator), 0);

    // Parent credit should be fully restored to 100
    let parent = market.get_credit(&credit_id);
    assert_eq!(parent.carbon_amount, 100);
}

#[test]
#[should_panic(expected = "Insufficient carbon amount available")]
fn test_buy_insufficient_credits() {
    let (env, market, admin, _, token_client, token_admin) = setup_test();
    let creator = Address::generate(&env);
    let buyer = Address::generate(&env);
    
    token_admin.mint(&buyer, &10_000);
    let credit_id = market.create_credit(&creator, &String::from_str(&env, "Forest Project"), &50);
    market.verify_credit(&admin, &credit_id, &VerificationStatus::Verified);
    market.list_credit(&creator, &credit_id, &50);

    // Attempting to buy 100 tons when only 50 are available should PANIC
    market.buy_credit(&buyer, &credit_id, &100); 
}

#[test]
#[should_panic(expected = "Only admin can verify")]
fn test_unauthorized_access() {
    let (env, market, _admin, _, _, _) = setup_test();
    let creator = Address::generate(&env);
    let malicious_user = Address::generate(&env);
    
    let credit_id = market.create_credit(&creator, &String::from_str(&env, "Ocean Project"), &100);
    
    // Attempting to verify the credit as a malicious_user, instead of the admin, should PANIC
    market.verify_credit(&malicious_user, &credit_id, &VerificationStatus::Verified);
}

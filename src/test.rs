#![cfg(test)]
use soroban_sdk::{testutils::Address as _, Env, Address, symbol_short};
use crate::{Crop_Pay, CropPayClient};

#[test]
fn happy_path_payment() {
    let env = Env::default();
    let contract_id = env.register(Crop_Pay, ());
    let client = Crop_PayClient::new(&env, &contract_id);

    let buyer = Address::generate(&env);
    let farmer = Address::generate(&env);

    // Buyer pays farmer
    client.pay(&buyer, &farmer, &100);

    // Verify payment stored
    assert_eq!(client.check(&buyer, &farmer), Some(100));
}

#[test]
fn unauthorized_payment_fails() {
    let env = Env::default();
    let contract_id = env.register(Crop_Pay, ());
    let client = Crop_PayClient::new(&env, &contract_id);

    let buyer = Address::generate(&env);
    let farmer = Address::generate(&env);

    // Farmer tries to call pay instead of buyer → should panic
    let result = std::panic::catch_unwind(|| {
        client.pay(&farmer, &buyer, &50);
    });
    assert!(result.is_err());
}

#[test]
fn state_verification() {
    let env = Env::default();
    let contract_id = env.register(Crop_Pay, ());
    let client = Crop_PayClient::new(&env, &contract_id);

    let buyer = Address::generate(&env);
    let farmer = Address::generate(&env);

    client.pay(&buyer, &farmer, &200);

    // Confirm correct state in storage
    assert_eq!(client.check(&buyer, &farmer), Some(200));
}
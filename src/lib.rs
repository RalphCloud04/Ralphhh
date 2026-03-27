#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Address, Symbol, symbol_short};

#[contract]
pub struct CropPay;

#[contractimpl]
impl CropPay {
    const KEY: Symbol = symbol_short!("harvest");

    pub fn pay(env: Env, buyer: Address, farmer: Address, amount: i128) {
        buyer.require_auth();
        env.storage().persistent().set(&(Self::KEY, buyer, farmer), &amount);
    }

    pub fn check(env: Env, buyer: Address, farmer: Address) -> Option<i128> {
        env.storage().persistent().get(&(Self::KEY, buyer, farmer))
    }
}

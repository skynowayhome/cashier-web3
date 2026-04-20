#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Receipt {
    pub tx_id: u32,
    pub amount: u32,
    pub cashier: String,
}

const RECEIPTS: Symbol = symbol_short!("RECEIPTS");

#[contract]
pub struct PosContract;

#[contractimpl]
impl PosContract {
    // Fungsi untuk mencatat transaksi ke blockchain
    pub fn save(env: Env, tx_id: u32, amount: u32, cashier: String) {
        let mut data: Vec<Receipt> = env.storage().instance().get(&RECEIPTS).unwrap_or(Vec::new(&env));
        data.push_back(Receipt { tx_id, amount, cashier });
        env.storage().instance().set(&RECEIPTS, &data);
    }

    // Fungsi untuk melihat semua transaksi (audit)
    pub fn get_all(env: Env) -> Vec<Receipt> {
        env.storage().instance().get(&RECEIPTS).unwrap_or(Vec::new(&env))
    }
}
#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, log};

// ----------------------
// Data Structures
// ----------------------

#[contracttype]
#[derive(Clone)]
pub struct Payment {
    pub payment_id: u64,
    pub user: String,
    pub creator: String,
    pub amount: u64,
    pub timestamp: u64,
}

// Use a simple string-based key instead of symbol_short! macro
const COUNT_PAYMENT: &str = "P_COUNT";

// ----------------------
// Contract Definition
// ----------------------
#[contract]
pub struct ARVRMicropayments;

// ----------------------
// Contract Implementation
// ----------------------
#[contractimpl]
impl ARVRMicropayments {

    // Function to create a new micropayment record
    pub fn make_payment(env: Env, user: String, creator: String, amount: u64) -> u64 {
        let mut count_payment: u64 = env.storage().instance().get(&Symbol::new(&env, COUNT_PAYMENT)).unwrap_or(0);
        count_payment += 1;

        let timestamp = env.ledger().timestamp();

        let payment = Payment {
            payment_id: count_payment,
            user,
            creator,
            amount,
            timestamp,
        };

        // Store payment using the payment_id directly as a u64 key
        env.storage().instance().set(&count_payment, &payment);
        env.storage().instance().set(&Symbol::new(&env, COUNT_PAYMENT), &count_payment);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Micropayment Created with ID: {}", count_payment);

        count_payment
    }

    // Function to view details of a specific payment using its ID
    pub fn view_payment(env: Env, payment_id: u64) -> Payment {
        env.storage().instance().get(&payment_id).unwrap_or(Payment {
            payment_id: 0,
            user: String::from_str(&env, "Not Found"),
            creator: String::from_str(&env, "Not Found"),
            amount: 0,
            timestamp: 0,
        })
    }

    // Function to view the total number of micropayments made
    pub fn view_total_payments(env: Env) -> u64 {
        env.storage().instance().get(&Symbol::new(&env, COUNT_PAYMENT)).unwrap_or(0)
    }
}
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, Promise};
use near_sdk::{AccountId};
use near_sdk::collections::{UnorderedMap};

mod flight;
mod baggage;
mod types;
mod fee;

use crate::{flight::Flight};
use crate::types::*;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    owner: AccountId,
    payment_account: AccountId,
    initialized: bool,

    user_flights: UnorderedMap<AccountId, UnorderedMap<FlightId, Flight>>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            owner: env::predecessor_account_id(),
            payment_account: env::predecessor_account_id(),
            initialized: false,
            user_flights: UnorderedMap::new(b"user_flights".to_vec()),
        }
    }
}

#[near_bindgen]
impl Contract {

    fn debug(&self) {
        let signer = env::signer_account_id();
        let predecessor = env::predecessor_account_id();
        let current = env::current_account_id();

        env::log(format!("Signer: {}",&signer).as_bytes());
        env::log(format!("Predecessor: {}",&predecessor).as_bytes());
        env::log(format!("Current: {}",&current).as_bytes());
    }

    // ====================================================================
    pub fn get_payment_account(&self) -> &String {
        &self.payment_account
    }

    pub fn get_initialized(&self) -> bool {
        self.initialized
    }

    // ====================================================================
    pub fn init(&mut self, payment_account: AccountId) {
        // Contract must not be initialized
        assert_eq!(
            self.initialized,
            false,
            "Contract already initialized"
        );

        // Only owner can init
        let predecessor = env::predecessor_account_id(); 
        assert_eq!(
            self.owner,
            predecessor,
            "Only contract owner can init"
        );
        
        // Payment account (Airlines) must not same as contract account
        let current = env::current_account_id();
        assert_ne!(
            payment_account,
            current,
            "Payment account cannot be contract"
        );

        self.initialized = true;
        self.payment_account = payment_account;
    
        env::log(format!(
            "Contract initialized by {}, payment to {}",
            predecessor,
            self.payment_account
        ).as_bytes());
    }

    pub fn reset(&mut self) {
        self.assert_initialized();

        let predecessor = env::predecessor_account_id();
        assert_eq!(
            self.owner,
            predecessor,
            "Only the contract owner can reset"
        );

        self.initialized = false;
        self.payment_account = self.owner.clone();
        self.user_flights.clear();
    
        env::log("Reset successfully".as_bytes());
    }

    // ====================================================================

    pub fn registry(&mut self, flight_id: FlightId) {
        let account_id = env::predecessor_account_id();
    }

    pub fn add_baggage(&mut self, flight_id: FlightId, baggage_size: Weight) {
        // Each (account, flight) can only have at most 3 baggages
        let customer = env::predecessor_account_id();
        
        // if (self.user_flights
        //     .get(&customer)
        //     .get(flight_id)
        //     .len()>3
        // ) {

        // }
    }

    pub fn check_baggages(&mut self, flight_id: FlightId) {

    }

    pub fn check_price(&mut self, flight_id: FlightId) {

    }

    #[payable]
    pub fn accept(&mut self, flight_id: FlightId) {
        let amount = env::attached_deposit();
    }

    pub fn remove_baggage(&mut self, flight_id: FlightId, baggage_id: BaggageId) {

    }

    pub fn remove_all_baggages(&mut self, flight_id: FlightId) {

    }

    pub fn transport_baggage(&mut self, customer_id: AccountId, flight_id: FlightId) {

    }

    #[payable]
    pub fn claim_baggages(&mut self, flight_id: FlightId) {

        self.remove_all_baggages(flight_id);

        let account_id = env::predecessor_account_id();
        // let amount_payment = self.user_flights.get(&account_id).get;
        // Promise::new(self.payment_account.to_string()).transfer(amount_payment);
    }

    // ===============================================
    fn assert_initialized(&self) {
        assert_eq!(
            self.initialized,
            true,
            "Contract was not initialized"
        );
    }
}
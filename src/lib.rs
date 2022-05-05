use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, Promise};
use near_sdk::{serde::{Serialize, Deserialize}, AccountId};
use near_sdk::collections::{UnorderedMap};

mod flight;
mod baggage;
mod types;
mod fee;

use crate::{flight::Flight, baggage::Baggage};
use crate::types::*;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    owner: AccountId,
    payment_account: AccountId,

    user_flights: UnorderedMap<AccountId, UnorderedMap<FlightId, Flight>>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            owner: env::signer_account_id(),
            payment_account: env::current_account_id(),
            user_flights: UnorderedMap::new(b"user_flights".to_vec()),
        }

    }
}

#[near_bindgen]
impl Contract {
    pub fn reset(&mut self) {
        env::log(b"Reset counter to zero");
    }

    pub fn init(&mut self, payment_account: AccountId) {

    }

    pub fn registry(&mut self, flight_id: FlightId) {
        let account_id = env::signer_account_id();
    }

    pub fn add_baggage(&mut self, flight_id: FlightId, baggage_size: Weight) {

    }

    pub fn check_baggages(&mut self, flight_id: FlightId) {

    }

    pub fn check_price(&mut self, flight_id: FlightId) {

    }

    pub fn accept(&mut self, flight_id: FlightId) {

    }

    pub fn remove_baggage(&mut self, flight_id: FlightId, baggage_id: BaggageId) {

    }

    pub fn remove_all(&mut self, flight_id: FlightId) {

    }

    pub fn claim_baggages(&mut self, flight_id: FlightId) {

        self.remove_all(flight_id);
    }
}
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, Promise};
use near_sdk::{AccountId};
use near_sdk::collections::{UnorderedMap};

mod flight;
mod baggage;
mod types;
mod fee;

use crate::flight::*;
use crate::baggage::*;
use crate::types::*;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    owner: AccountId,
    payment_account: AccountId,
    initialized: bool,
    user_flights: UnorderedMap<(AccountId, FlightId), FlightDetail>,
    count_baggage: BaggageId
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            owner: env::signer_account_id(),
            payment_account: env::predecessor_account_id(),
            initialized: false,
            user_flights: UnorderedMap::new(b"user_flights".to_vec()),
            count_baggage: 0
        }
    }
}

#[near_bindgen]
impl Contract {

    fn _debug(&self) {
        self.assert_initialized();

        let signer = env::signer_account_id();
        let predecessor = env::predecessor_account_id();
        let current = env::current_account_id();

        env::log(format!("Signer: {}",&signer).as_bytes());
        env::log(format!("Predecessor: {}",&predecessor).as_bytes());
        env::log(format!("Current: {}",&current).as_bytes());
    }

    // ====================================================================
    pub fn get_payment_account(&self) -> &String {
        self.assert_initialized();
        &self.payment_account
    }

    pub fn get_initialized(&self) -> bool {
        self.assert_initialized();
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
        self.user_flights = UnorderedMap::new(b"user_flights".to_vec());
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
    pub fn registry(
        &mut self, 
        flight_id: FlightId,
        flight_class: FlightClass,
        distance: Distance
    ) {
        self.assert_initialized();

        let customer = env::predecessor_account_id();
        let key = (customer, flight_id);
        match self.user_flights.get(&key) {
            Some(_flight) => {
                panic!("This flight was registered by you");
            },
            None => {
                let new_flight = FlightDetail::new(
                    flight_id,
                    flight_class,
                    distance
                );
                self.user_flights.insert(&key, &new_flight);
                env::log("Registry successfully".as_bytes());
            }
        }
    }

    pub fn add_baggage(&mut self, flight_id: FlightId, baggage_weight: Weight) {
        self.assert_initialized();
    
        let customer_id = env::predecessor_account_id();
        let key = &(customer_id, flight_id);

        match self.user_flights.get(&key) {
            Some(mut flight) => {
                        
                // Each (account, flight) can only have at most 3
                let baggage_len: usize = flight.get_baggages().len() as usize;
                if baggage_len>=3 {
                    panic!("You cannot add more than 3 baggages"); 
                } else {
                    let baggage_id: BaggageId = self.count_baggage;
                    let new_baggage = Baggage::new (
                        baggage_id,
                        baggage_weight
                    );
                    env::log(format!("Baggage size: {}",flight.get_baggages().len()).as_bytes());

                    flight.add_baggage(
                        new_baggage
                    );
                    self.user_flights.insert(&key,&flight);
                    self.count_baggage += 1;

                    env::log("Add baggage succesfully".as_bytes());
                    env::log(format!("Baggage size: {}",flight.get_baggages().len()).as_bytes());
                    env::log(format!("Baggage id: {}",&baggage_id).as_bytes());
                }
            },
            None => {
                panic!("Cannot find your flight");
            }
        }
    }

    pub fn check_baggages(&mut self, flight_id: FlightId) {
        self.assert_initialized();

        let customer_id = env::predecessor_account_id();
        let key = &(customer_id, flight_id);
        match self.user_flights.get(&key) {
            Some(flight) => {
                let baggages = flight.get_baggages();

                env::log(format!("Number of baggages: {}",baggages.len()).as_bytes());
                for baggage in baggages.values(){
                    env::log(format!(
                        "[{}] Weight={}", 
                        baggage.get_id(), 
                        baggage.get_weight()
                    ).as_bytes());
                }
            },
            None => {
                panic!("Cannot find your flight");
            }
        }
    }

    pub fn check_price(&mut self, flight_id: FlightId) {
        self.assert_initialized();
        
        let customer_id = env::predecessor_account_id();
        let key = &(customer_id, flight_id);

        match self.user_flights.get(&key) {
            Some(flight) => {        
                env::log(format!("Your price: {}NEAR",flight.get_price()).as_bytes());
            },
            None => {
                panic!("Cannot find your flight");
            }
        }
    }

    #[payable]
    pub fn accept(&mut self, flight_id: FlightId) {
        self.assert_initialized();

        let customer_id = env::predecessor_account_id();
        let key = &(customer_id, flight_id);

        match self.user_flights.get(&key) {
            Some(mut flight) => {        
                let fee = flight.get_price();
                let deposit = env::attached_deposit();
                assert_eq!(
                    fee,
                    deposit,
                    "You must pay {}NEAR", fee
                );

                flight.set_state(FlightState::Checked);
                env::log("Your baggages are checked".as_bytes());
            },
            None => {
                panic!("Cannot find your flight");
            }
        }

    }

    pub fn remove_baggage(&mut self, flight_id: FlightId, baggage_id: BaggageId) {
        self.assert_initialized();

        let customer_id = env::predecessor_account_id();
        let key = &(customer_id, flight_id);

        match self.user_flights.get(&key) {
            Some(mut flight) => {        
                flight.remove_baggage(baggage_id);
                env::log("Remove baggage successfully".as_bytes());
            },
            None => {
                panic!("Cannot find your flight");
            }
        }
    }

    pub fn remove_all_baggages(&mut self, flight_id: FlightId) {
        self.assert_initialized();

        let customer_id = env::predecessor_account_id();
        let key = &(customer_id, flight_id);

        match self.user_flights.get(&key) {
            Some(mut flight) => {        
                flight.clear_baggages();
                env::log("Remove all baggages successfully".as_bytes());
            },
            None => {
                panic!("Cannot find your flight");
            }
        }
    }

    pub fn transport_baggage(&mut self, customer_id: AccountId, flight_id: FlightId) {
        self.assert_initialized();

        let key = &(customer_id, flight_id);

        match self.user_flights.get(&key) {
            Some(mut flight) => {        
                flight.set_state(FlightState::Transported)
            },
            None => {
                panic!("Cannot find your flight");
            }
        }
    }

    #[payable]
    pub fn claim_baggages(&mut self, flight_id: FlightId) {
        self.assert_initialized();

        let customer_id = env::predecessor_account_id();
        let key = &(customer_id, flight_id);

        match self.user_flights.get(&key) {
            Some(mut flight) => {        
                self.remove_all_baggages(flight_id);
                flight.set_state(FlightState::Claimed);

                let payment_id = self.get_payment_account();
                let amount = flight.get_price();

                Promise::new(payment_id.to_string()).transfer(amount);
            },
            None => {
                panic!("Cannot find your flight");
            }
        }

    }

    pub fn get_count_list(&self) -> usize{
        self.user_flights.len() as usize
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
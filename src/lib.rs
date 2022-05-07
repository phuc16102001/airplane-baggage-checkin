use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, Promise, Balance};
use near_sdk::{AccountId};
use near_sdk::collections::{UnorderedMap};

pub mod flight_detail;
pub mod baggage;
pub mod types;
pub mod fee;

use crate::flight_detail::*;
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
        self.initialized
    }

    pub fn get_owner(&self) -> &String {
        &self.owner
    }
    // ====================================================================
    pub fn init(&mut self, payment_account: AccountId) {
        // Contract must not be initialized
        assert_eq!(
            self.initialized,
            false,
            "Contract was already initialized"
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

    pub fn add_baggage(
        &mut self, 
        flight_id: FlightId, 
        baggage_weight: Weight
    ) -> BaggageId {
        self.assert_initialized();
    
        let customer_id = env::predecessor_account_id();
        let key = &(customer_id, flight_id);

        match self.user_flights.get(&key) {
            Some(mut flight) => {
                self.assert_state(&flight, FlightState::Idle);
                        
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

                    flight.add_baggage(
                        new_baggage
                    );
                    self.user_flights.insert(&key,&flight);
                    self.count_baggage += 1;

                    env::log("Add baggage succesfully".as_bytes());
                    env::log(format!("Baggage id: {}",&baggage_id).as_bytes());

                    baggage_id
                }
            },
            None => {
                panic!("Cannot find your flight");
            }
        }
    }

    pub fn check_number_baggages(
        &mut self, 
        flight_id: FlightId
    ) -> u64 {
        self.assert_initialized();

        let customer_id = env::predecessor_account_id();
        let key = &(customer_id, flight_id);
        match self.user_flights.get(&key) {
            Some(flight) => {
                let baggages = flight.get_baggages();
                env::log(format!("Number of baggages: {}",baggages.len()).as_bytes());
                baggages.len()
            },
            None => {
                panic!("Cannot find your flight");
            }
        }
    }

    pub fn check_baggage(
        &mut self, 
        flight_id: FlightId,
        baggage_id: BaggageId
    ) -> Baggage {
        self.assert_initialized();

        let customer_id = env::predecessor_account_id();
        let key = &(customer_id, flight_id);

        match self.user_flights.get(&key) {
            Some(flight) => {
                let baggages = flight.get_baggages();
                
                match baggages.get(&baggage_id) {
                    Some(baggage) => {
                        env::log(format!(
                            "[{}] Weight={}", 
                            baggage.get_id(), 
                            baggage.get_weight()
                        ).as_bytes());
                        baggage
                    },
                    None => {
                        panic!("Cannot find your baggage");
                    }
                }
            },
            None => {
                panic!("Cannot find your flight");
            }
        }
    }

    pub fn check_fee(&mut self, flight_id: FlightId) -> Balance {
        self.assert_initialized();
        
        let customer_id = env::predecessor_account_id();
        let key = &(customer_id, flight_id);

        match self.user_flights.get(&key) {
            Some(flight) => {        
                let price = flight.get_fee();
                env::log(format!("Your price: {} NEAR",&price).as_bytes());
                price
            },
            None => {
                panic!("Cannot find your flight");
            }
        }
    }

    pub fn check_state(&mut self, flight_id: FlightId) -> String {
        self.assert_initialized();
        
        let customer_id = env::predecessor_account_id();
        let key = &(customer_id, flight_id);

        match self.user_flights.get(&key) {
            Some(flight) => {        
                env::log(format!("State: {:?}",flight.get_state()).as_bytes());
                format!("{:?}",&flight.get_state())
            },
            None => {
                panic!("Cannot find your flight");
            }
        }
    }

    
    pub fn check_class(&mut self, flight_id: FlightId) -> String {
        self.assert_initialized();
        
        let customer_id = env::predecessor_account_id();
        let key = &(customer_id, flight_id);

        match self.user_flights.get(&key) {
            Some(flight) => {        
                let flight_class = flight.get_flight_class();
                env::log(format!("Class: {:?}",&flight_class).as_bytes());
                format!("{:?}",&flight_class)
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
                self.assert_state(
                    &flight, 
                    FlightState::Idle,
                );
                let fee = flight.get_fee();
                let deposit = env::attached_deposit();
                assert_eq!(
                    to_yoto(fee),
                    deposit,
                    "You must pay {} NEAR", fee
                );

                flight.set_state(FlightState::Checked);
                self.user_flights.insert(&key,&flight);
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
                self.assert_state(&flight, FlightState::Idle);
                flight.remove_baggage(baggage_id);
                self.user_flights.insert(&key,&flight);
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
                self.user_flights.insert(&key,&flight);
                env::log("Remove all baggages successfully".as_bytes());
            },
            None => {
                panic!("Cannot find your flight");
            }
        }
    }

    pub fn deliver_baggage(&mut self, customer_id: AccountId, flight_id: FlightId) {
        self.assert_initialized();

        assert_eq!(
            env::predecessor_account_id(),
            self.payment_account,
            "Only airline can deliver the baggages"
        );

        let key = &(customer_id, flight_id);

        match self.user_flights.get(&key) {
            Some(mut flight) => {        
                self.assert_state(&flight, FlightState::Checked);
                flight.set_state(FlightState::Delivered);
                self.user_flights.insert(&key,&flight);
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
                self.assert_state(&flight, FlightState::Delivered);

                flight.set_state(FlightState::Claimed);
                self.user_flights.insert(&key,&flight);
                
                Promise::new(
                    self.get_payment_account().to_string()
                ).transfer(
                    to_yoto(flight.get_fee())
                );
            },
            None => {
                panic!("Cannot find your flight");
            }
        }

    }

    // ===============================================
    fn assert_initialized(&self) {
        assert_eq!(
            self.initialized,
            true,
            "Contract was not initialized"
        );  

    }

    fn assert_state(&self, flight: &FlightDetail, target_state: FlightState) {
        if !(*flight.get_state() == target_state) {
            panic!(
                "You can only do this in {:?} state, not {:?}",
                target_state,
                flight.get_state()
            );
        }
    }
}
use near_sdk::{serde::{Serialize, Deserialize}};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;

use crate::fee::*;
use crate::types::*;
use crate::baggage::Baggage;

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum FlightClass {
    First,
    Business,
    Economy
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Flight {
    _flight_id: FlightId,
    _flight_class: FlightClass, 
    _distance: Distance,
    _baggages: UnorderedMap<BaggageId, Baggage>,
}

impl Flight {
    pub fn new(
        _flight_id: FlightId,
        _flight_class: FlightClass,
        _distance: Distance,
    ) -> Self {
        Self {
            _flight_id,
            _flight_class,
            _distance,
            _baggages: UnorderedMap::new(b"baggages".to_vec())
        }
    }

    pub fn get_flight_id(&self) -> FlightId{
        self._flight_id
    }
    
    pub fn get_flight_class(&self) -> &FlightClass{
        &self._flight_class
    }
    
    pub fn get_distance(&self) -> Distance{
        self._distance
    }
    
    pub fn get_baggages(&self) -> &UnorderedMap<BaggageId, Baggage>{
        &self._baggages
    }

    pub fn get_price(&self) -> Price {
        self.get_fee_strategy().calculate_fee(
            self._distance,
            &self._baggages
        )
    }

    fn get_fee_strategy(&self) -> Box<dyn FeeStrategy> {
        match &self._flight_class {
            FlightClass::First => Box::new(FirstFee{}),
            FlightClass::Business => Box::new(BusinessFee{}),
            FlightClass::Economy => Box::new(EconomyFee{})
        }
    }
}
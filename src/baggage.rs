use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};  
use crate::types::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Baggage {
    baggage_id: BaggageId,
    baggage_weight: Weight
}

impl Baggage {
    pub fn new(baggage_id: BaggageId, baggage_weight: Weight) -> Self {
        Self {
            baggage_id,
            baggage_weight
        }
    }

    pub fn get_id(&self) -> BaggageId {
        self.baggage_id
    }
    
    pub fn get_weight(&self) -> Weight {
        self.baggage_weight
    }
}
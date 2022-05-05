use near_sdk::collections::UnorderedMap;
  
use crate::{types::*, baggage::Baggage};

pub trait FeeStrategy {
    fn calculate_fee(
        &self,
        _distance: Distance,
        _baggages: &UnorderedMap<BaggageId, Baggage>
    ) -> Price;
}

pub struct FirstFee {}
impl FeeStrategy for FirstFee {
    fn calculate_fee(
        &self,
        _distance: Distance,
        _baggages: &UnorderedMap<BaggageId, Baggage>
    ) -> Price {
        1
    }
}

pub struct BusinessFee {}
impl FeeStrategy for BusinessFee {
    fn calculate_fee(
        &self,
        _distance: Distance,
        _baggages: &UnorderedMap<BaggageId, Baggage>
    ) -> Price {
        1
    }
}

pub struct EconomyFee {}
impl FeeStrategy for EconomyFee {
    fn calculate_fee(
        &self,
        _distance: Distance,
        _baggages: &UnorderedMap<BaggageId, Baggage>
    ) -> Price {
        1
    }
}
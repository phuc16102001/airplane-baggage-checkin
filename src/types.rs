use near_sdk::Balance;

pub type BaggageId = u64;
pub type FlightId = u64;
pub type Distance = f32;    // miles    
pub type Weight = f32;  // pound (lbs)


pub fn to_yoto(near: Balance) -> Balance {
    return near*10u128.pow(24);
}
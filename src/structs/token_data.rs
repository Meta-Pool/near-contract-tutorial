use near_sdk::{json_types::U128, near};

#[near(serializers = [json, borsh])]
pub struct TokenData {
    pub amount: U128,
    pub decimals: u8,
}

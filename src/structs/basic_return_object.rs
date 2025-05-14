use near_sdk::near;

#[near(serializers = [json, borsh])]
pub struct BasicReturnObject {
    pub greeting: String,
    pub number: u8,
}

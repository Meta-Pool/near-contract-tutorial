// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{log, near_bindgen, PanicOnDefault};
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct BasicReturnObject {
    pub greeting: String,
    pub number: u8,
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct NearTutorialContract {
    greeting: String,
}

// Implement the contract structure
#[near_bindgen]
impl NearTutorialContract {
    #[init]
    pub fn new(initial_greeting: String) -> Self {
        Self {
            greeting: initial_greeting,
        }
    }

    // Public method - returns the greeting saved, defaulting to DEFAULT_GREETING
    pub fn get_greeting(&self) -> String {
        self.greeting.clone()
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn set_greeting(&mut self, greeting: String) {
        log!("Saving greeting: {greeting}");
        self.greeting = greeting;
    }

    pub fn return_u8(&self) -> u8 {
        // This is a simple function that returns a u8 value. Same applies for u16, u32 and u64. Problem can arise when returning a value with scientific notation
        // Js starts having issues with numbers of the order of 10^21, and u64 is of the order of 10^19 so it's ok. The issue will begin with u128
        42
    }

    pub fn return_u128_badly(&self) -> u128 {
        // Check that rust test will pass smoothly, but on js or bash it will be with scientific notation
        42_000_000_000_000_000_000_000_000_000_000
    }

    pub fn return_u128(&self) -> U128 {
        // js will take this as a string
        U128(42_000_000_000_000_000_000_000_000_000_000)
    }

    pub fn return_str_ref(&self) -> &str {
        // this will return a reference to the string, but it will be dropped after the function ends
        // so it will not be usable outside of this function
        &self.greeting
    }

    pub fn return_bool(&self) -> bool {
        // this will return a copy of the string, so it will be usable outside of this function
        true
    }

    pub fn return_obj(&self) -> BasicReturnObject {
        // this will return a copy of the string, so it will be usable outside of this function
        BasicReturnObject {
            greeting: self.greeting.clone(),
            number: 42,
        }
    }

    pub fn return_vec(&self, from: u8, limit: usize) -> Vec<u8> {
        // Normally, when returning a vector, we will have this 2 parameters since the object may be too big and take long. Normally, limit is 100
        // There are other ways to define a vector, but it is out of the scope of this tutorial
        (from..from + limit as u8).collect()
    }

    pub fn return_option_none(&self) -> Option<u8> {
        // this will return a None value
        None
    }
    pub fn return_option_some(&self) -> Option<u8> {
        // this will return a Some value
        Some(42)
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use near_sdk::test_utils::VMContextBuilder;

    use super::*;

    fn setup_contract() -> (VMContextBuilder, NearTutorialContract) {
        let context = VMContextBuilder::new();
        let contract = NearTutorialContract::new("Hello".to_string());
        (context, contract)
    }

    #[test]
    fn get_default_greeting() {
        let (_, contract) = setup_contract();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(contract.get_greeting(), "Hello");
    }

    #[test]
    fn set_then_get_greeting() {
        let (_, mut contract) = setup_contract();
        contract.set_greeting("howdy".to_string());
        assert_eq!(contract.get_greeting(), "howdy");
    }

    #[test]
    fn get_u8() {
        let (_, contract) = setup_contract();
        assert_eq!(contract.return_u8(), 42);
    }

    #[test]
    fn get_u128_badly() {
        let (_, contract) = setup_contract();
        assert_eq!(
            contract.return_u128_badly(),
            42_000_000_000_000_000_000_000_000_000_000
        );
    }

    #[test]
    fn get_u128() {
        let (_, contract) = setup_contract();
        assert_eq!(
            contract.return_u128(),
            U128(42_000_000_000_000_000_000_000_000_000_000)
        );
    }

    #[test]
    fn get_str_ref() {
        let (_, contract) = setup_contract();
        assert_eq!(contract.return_str_ref(), &"Hello".to_string());
    }

    #[test]
    fn get_bool() {
        let (_, contract) = setup_contract();
        assert_eq!(contract.return_bool(), true);
    }

    #[test]
    fn get_obj() {
        let (_, contract) = setup_contract();
        let obj = contract.return_obj();
        assert_eq!(obj.greeting, "Hello");
        assert_eq!(obj.number, 42);
    }

    #[test]
    fn get_vec() {
        let (_, contract) = setup_contract();
        let vec = contract.return_vec(1, 10);
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(vec, expected);
    }

    #[test]
    fn get_option_none() {
        let (_, contract) = setup_contract();
        let vec = contract.return_option_none();
        assert_eq!(vec.is_none(), true);
    }

    #[test]
    fn get_option_some() {
        let (_, contract) = setup_contract();
        let vec = contract.return_option_some();
        assert_eq!(vec.unwrap(), 42);
    }
}

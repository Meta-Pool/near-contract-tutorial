// Find all our documentation at https://docs.near.org
use near_sdk::{log, near, near_bindgen, PanicOnDefault};

// Define the contract structure
#[near(contract_state)]
#[derive(PanicOnDefault)]
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
}

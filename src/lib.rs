// Find all our documentation at https://docs.near.org
use near_sdk::{
    assert_one_yocto, env, json_types::U128, log, near, near_bindgen, require, AccountId,
    PanicOnDefault,
};
use uint::construct_uint;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

#[near(serializers = [json, borsh])]
pub struct BasicReturnObject {
    pub greeting: String,
    pub number: u8,
}
#[near(serializers = [json, borsh])]
pub struct TokenData {
    pub amount: U128,
    pub decimals: u8,
}

pub(crate) fn proportional(amount: u128, numerator: u128, denominator: u128) -> u128 {
    (U256::from(amount) * U256::from(numerator) / U256::from(denominator)).as_u128()
}

// Define the contract structure
#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct NearTutorialContract {
    owner: String,
}

#[near(serializers = [json, borsh])]
pub struct OldNearTutorialContract {
    greeting: String,
}

// Implement the contract structure
#[near_bindgen]
impl NearTutorialContract {
    #[init]
    pub fn new(initial_owner: String) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner: initial_owner,
        }
    }

    #[init(ignore_state)]
    pub fn migrate(new_owner: String) -> Self {
        // We will not be using the old state, but normally we would need to migrate everything that is relevante to the new state
        let _old_state: OldNearTutorialContract = env::state_read().expect("Failed to read state");

        let new_state = Self { owner: new_owner };

        env::state_write(&new_state);

        new_state
    }

    // This function is used to check if the caller is the owner of the contract
    // Currently, the owner is hardcoded to "near-tuto.testnet"
    // In the future, we will use a better way to manage the owner
    fn assert_owner(&self) {
        require!(
            env::predecessor_account_id()
                == AccountId::from("near-tuto.testnet".parse::<AccountId>().unwrap()),
            "Not the owner"
        );
    }

    // Public method - returns the greeting saved, defaulting to DEFAULT_GREETING
    pub fn get_greeting(&self) -> String {
        self.greeting.clone()
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    #[payable]
    pub fn set_greeting(&mut self, greeting: String) {
        assert_one_yocto();
        self.assert_owner();
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

    pub fn bad_divide(&self, a: u8, b: u8) -> u8 {
        // this function should have imprecitions
        require!(b != 0, "Division by zero");
        a / b
    }

    pub fn divide_token(&self, a: TokenData, b: TokenData) -> TokenData {
        // this may fail due to returning a json with a U128 value
        require!(b.amount.0 != 0, "Division by zero");
        let res = proportional(a.amount.0, 10_u128.pow(b.decimals as u32), b.amount.0);
        TokenData {
            amount: U128(res),
            decimals: a.decimals,
        }
    }

    pub fn percent(&self, a: U128, b: u16) -> U128 {
        // this may fail due to returning a json with a U128 value
        require!(b != 0, "Division by zero");
        require!(b <= 10000, "Percentage too high");
        U128(proportional(a.0, b as u128, 10000))
    }

    pub fn log(&self) {
        log!("Greeting: {}", self.greeting);
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use near_sdk::{test_utils::VMContextBuilder, testing_env, NearToken};

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
        let (mut ctx, mut contract) = setup_contract();
        ctx.predecessor_account_id("near-tuto.testnet".parse::<AccountId>().unwrap())
            .attached_deposit(NearToken::from_yoctonear(1));
        testing_env!(ctx.build());

        contract.set_greeting("howdy".to_string());
        assert_eq!(contract.get_greeting(), "howdy");
    }

    #[test]
    #[should_panic(expected = "Requires attached deposit of exactly 1 yoctoNEAR")]
    fn set_greeting_should_panic() {
        let (mut ctx, mut contract) = setup_contract();
        ctx.predecessor_account_id("near-tuto.testnet".parse::<AccountId>().unwrap());
        testing_env!(ctx.build());

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

    #[test]
    fn test_bad_divide() {
        let (_, contract) = setup_contract();
        let res = contract.bad_divide(5, 2);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_divide_token_same_decimals() {
        let (_, contract) = setup_contract();
        let res = contract.divide_token(
            TokenData {
                amount: U128(5_000_000_000_000_000_000_000_000),
                decimals: 24,
            },
            TokenData {
                amount: U128(2_000_000_000_000_000_000_000_000),
                decimals: 24,
            },
        );
        let expected_res = TokenData {
            amount: U128(2_500_000_000_000_000_000_000_000),
            decimals: 24,
        };
        assert_eq!(res.amount, expected_res.amount);
        assert_eq!(res.decimals, 24);
    }

    #[test]
    fn test_divide_token_different_decimals_bigger() {
        let (_, contract) = setup_contract();
        let res = contract.divide_token(
            TokenData {
                amount: U128(5_000_000_000_000_000_000_000_000),
                decimals: 24,
            },
            TokenData {
                amount: U128(2_000_000),
                decimals: 6,
            },
        );
        let expected_res = TokenData {
            amount: U128(2_500_000_000_000_000_000_000_000),
            decimals: 24,
        };
        assert_eq!(res.amount, expected_res.amount);
        assert_eq!(res.decimals, 24);
    }

    #[test]
    fn test_divide_token_different_decimals_smaller() {
        let (_, contract) = setup_contract();
        let res = contract.divide_token(
            TokenData {
                amount: U128(5_000_000),
                decimals: 6,
            },
            TokenData {
                amount: U128(2_000_000_000_000_000_000_000_000),
                decimals: 24,
            },
        );
        let expected_res = TokenData {
            amount: U128(2_500_000),
            decimals: 24,
        };
        assert_eq!(res.amount, expected_res.amount);
        assert_eq!(res.decimals, 6);
    }

    #[test]
    fn test_get_1_percent() {
        let (_, contract) = setup_contract();
        let res = contract.percent(U128(2_000_000_000_000_000_000_000_000), 100);
        let expected_res = U128(20_000_000_000_000_000_000_000);
        assert_eq!(res.0, expected_res.0);
    }
}

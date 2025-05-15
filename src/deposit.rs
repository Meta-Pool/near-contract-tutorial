use crate::NearTutorialContract;
use crate::NearTutorialContractExt;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::log;
use near_sdk::{env, json_types::U128, near_bindgen, AccountId, PromiseOrValue};

#[near_bindgen]
impl FungibleTokenReceiver for NearTutorialContract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        let ft_token = env::predecessor_account_id();
        log!("Token type contract: {}", ft_token);
        log!("Sender: {}", sender_id);
        log!("Amount: {}", amount.0);
        log!("Message (it may help us decide a flow): {}", msg);

        // Tokens to return to the sender. It may be because user transferred more tokens than needed or we may return 0
        // A valid example of transferring more tokens could be if a user swaps tokens and maybe we ask a little bit more tokens to be sure the swap is successful
        // In this case we will return all the tokens because we are deleting the contract after manually testing it
        PromiseOrValue::Value(amount)
    }
}

use crate::*;

impl NearTutorialContract {
    // This function is used to check if the caller is the owner of the contract
    // Currently, the owner is hardcoded to "near-tuto-1.testnet"
    // In the future, we will use a better way to manage the owner
    pub(crate) fn assert_owner(&self) {
        require!(env::predecessor_account_id() == self.owner, "Not the owner");
    }
}

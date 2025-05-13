#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod balance_mapping_2 {

    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct BalanceMapping2 {
        /// Assign a balance to every account ID
        balances: Mapping<AccountId, Balance>,
    }
    // Compiler asked me to set this.So no warnings.
    impl Default for BalanceMapping2 {
         fn default() -> Self {
             Self::new()
         }
    }

    impl BalanceMapping2 {
       /// Constructor to initialize the contract with an empty mapping.
        #[ink(constructor, payable)]
        pub fn new() -> Self {
            let balances = Mapping::default();
            Self { balances }
        }

        /// Retrieve the balance of the caller.-> In solidity view the slot value should return 0 if account doesnt exist
        /// In mapping however it returns null here (options)
        #[ink(message)]
        pub fn get_balance(&self) -> Option<Balance> {
            let caller = self.env().caller();
            self.balances.get(caller)
        }

        /// Credit more money to the contract.
        #[ink(message, payable)]
        pub fn transfer(&mut self) {
            let caller = self.env().caller();
            let balance = self.balances.get(caller).unwrap_or(0);
            let endowment = self.env().transferred_value();

            // Safe add for overflow check.
            let new_balance = balance
                .checked_add(endowment)
                .expect("Balance overflow");
            self.balances.insert(caller, &new_balance);
        }
        /// Withdraw all your balance from the contract.
        #[ink(message)]
        pub fn withdraw(&mut self) {
            let caller = self.env().caller();
            let balance = self.balances.get(caller).unwrap();
            self.balances.remove(caller);
            self.env().transfer(caller, balance).unwrap()
        }
        /// Returns the `balance` of the contract.
        #[ink(message)]
        pub fn get_contract_balance(&self) -> Balance {
            self.env().balance()
        }
    }
}






/* 

        /// Withdraw all your balance from the contract.
        /// Returns true on success, false on failure
        #[ink(message)]
        pub fn withdraw(&mut self, wdraw_amount: Balance) -> bool {
        let caller = self.env().caller();

        match self.balances.get(caller) {
            Some(balance) => {
                if wdraw_amount > balance {
                     // This's debug line for checking out the reason of failure.
                    ink::env::debug_println!(
                        "Withdraw failed: not enough balance (have {}, tried to withdraw {})",
                        balance,
                        wdraw_amount
                    );
                    return false;
                }

            match self.env().transfer(caller, wdraw_amount) {
                Ok(()) => {
                    let remaining = balance
                .checked_sub(wdraw_amount)
                .expect("Balance overflow");
                    if remaining == 0 {
                        self.balances.remove(caller);
                    } else {
                        self.balances.insert(caller, &remaining);
                    }
                     true
                }
                Err(e) => {
                    // This's debug line for checking out the reason of failure.
                    ink::env::debug_println!("Transfer failed: {:?}", e);
                    false
                }
            }
        }
        None => {
            // This's debug line for checking out the reason of failure.
            ink::env::debug_println!("Withdraw failed: no balance for caller {:?}", caller);
            false
                }
            }
        }

      
    }
 */
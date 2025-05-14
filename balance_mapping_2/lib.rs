#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod balance_mapping_2 {

    use ink::storage::Mapping;

    use ink::env::{
        call::{build_call, ExecutionInput, Selector},
        DefaultEnvironment,
        };

    #[ink(storage)]
    pub struct BalanceMapping2 {
        /// Assign a balance to every account ID
        balances: Mapping<AccountId, Balance>,
        // contract we gonna use for remote flip. 
        flip_contract:AccountId,
    }

    /// Defines an event that is emitted
    /// every time value is transferred.
    #[ink(event)]
    pub struct Transferred {
        from: Option<AccountId>,
        to: Option<AccountId>,
        value: Balance,
    }

    /// Defines an event that is emitted
    /// every time remote function's invoked.
    #[ink(event)]
    pub struct RemoteInvoked {
        from: Option<AccountId>,
        to: Option<AccountId>,
    }

    /// Defines an event that is emitted
    /// every time withdraw function's invoked.
    #[ink(event)]
    pub struct Withdrawed {
        from: Option<AccountId>,
        to: Option<AccountId>,
        value:Balance,
    }
   
    impl BalanceMapping2 {
       /// Constructor to initialize the contract with an empty mapping.
        #[ink(constructor, payable)]
        pub fn new(contract_address:AccountId) -> Self {
            let balances = Mapping::default();
            let flip_contract= contract_address;
            Self { 
                   balances,
                   flip_contract 
                }
        }

        #[ink(message)]
        pub fn remote_flip(&mut self) {
            // get caller account-id.
            let caller =self.env().caller();

            Self::env().emit_event(RemoteInvoked{
                from:Some(caller),
                to:Some(self.flip_contract),
            });

            let _ = build_call::<DefaultEnvironment>()
            .call(self.flip_contract)
            .call_v1()
            .gas_limit(0)
            .transferred_value(0)
            .exec_input(
        ExecutionInput::new(Selector::new(ink::selector_bytes!("flip")))
            .push_arg(42u8)
            .push_arg(true)
            .push_arg(&[0x10u8; 32])
                         )
        .returns::<()>() // Not <bool>
        .invoke();

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

            Self::env().emit_event(Transferred{
                from:Some(caller),
                to:Some(self.flip_contract),
                value:endowment,
            });

            // Safe add for overflow check.
            let new_balance = balance
                .checked_add(endowment)
                .expect("Balance overflow");
            self.balances.insert(caller, &new_balance);
        }
        /// Withdraw all your balance from the contract.
        #[ink(message)]
        pub fn withdraw(&mut self) {
            // Get caller account address.
            let caller = self.env().caller();
            // Get balance value from map.
            let balance = self.balances.get(caller).unwrap();
            // fire withdraw event.
            Self::env().emit_event(Withdrawed{
                from:Some(self.env().account_id()),
                to:Some(caller),
                value:balance,
            });
            // remove callers value from map. 
            self.balances.remove(caller);
            // do the transfer.
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
## Here Lies Certain Documentation I find crucial to understand Ink Contracts.

### Get Starting 

lib.rs
Every ink! contract is required to contain:

Exactly one ```#[ink(storage)]``` struct.

At least one ```#[ink(constructor)]``` function.

At least one ```#[ink(message)]``` function.

```
#![cfg_attr(not(feature = "std"), no_std)] 
``` 
-> Above line is always present in Polkadot ink! smart contracts to ensure they compile for Wasm properly:

So that your smart contract can run both:

- In a normal Rust dev environment (with std) for testing.

- In Wasm for blockchain deployment (without std).

---> By default functions are private, they have to be annotated with `#[ink(message)]` and `pub` to be available from the outside.
A state-mutating function that the contract exposes to the outside world.
```
  #[ink(message)]
        pub fn flip(&mut self) {
            /* --snip-- */
        }
```

---> A public contract function that has no side-effects.These functions maybe invoked by transactions which leads to wasting funds so this are typically invoked via RPC to return a contract's state.
```
   #[ink(message)]
        pub fn get(&self) -> bool {
            /* --snip-- */
        }
```

### Storing Values

How to set up simple values in storage 

```
#[ink(storage)]
pub struct MyContract {
    // Store a bool
    my_bool: bool,
    // Store some number
    my_number: u32,
}
```
--> Furthermore, ink! provides substrate specific types like AccountId, Balance, and Hash to smart contracts as if they were primitive types.

--> The ink_prelude crate provides an efficient approach to import commonly used Rust types such as String and Vec, ensuring safe usage within an ink! contract.

```
#[ink::contract]
mod MyContractWithStringsAndArrays {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;

    #[ink(storage)]
    pub struct MyContract {
        // Store some String
        my_string: String,
        // Store some u32 in a vec -> growable array
        my_vector: Vec<u32>,
    }
    /* --snip-- */
}
```

--> Mapping 
Mapping much more efficient compare to other elements inside prelude package like hashmap.However other elements packed tightly for efficent storage usage.

```
#[ink(storage)]
pub struct MyContract {
    /// Assign a balance to every account.
    balances: ink::storage::Mapping<AccountId, Balance>,
}
```

!!!! The static buffer defaults to 16KB in size.
Consider a Mapping with String values like so:

```
#[ink(storage)]
pub struct MyContract {
    on_chain_log: Mapping<u64, String>,
    nonce: u64,
}
```
If the String overgrows the static buffer size, it will no longer fit into the mapping and it will panic

Instead, consider using the fallible try_insert method to handle the situation:
```
#[ink(message)]
pub fn do_something2(&mut self, data: String) {
    let caller = self.env().caller();

    let log_message = format!("{caller:?}: {data}");

    // `try_insert` will not panic but return an error instead.
    if self
        .on_chain_log
        .try_insert(&self.nonce, &log_message)
        .is_err()
    {
        // We get the chance to handle this problem properly:
        // Restrain the log message to a size guaranteed to fit.
        let log_message = format!("{caller:?}: <data omitted>");
        self.on_chain_log.insert(&self.nonce, &log_message);
    }

    self.nonce += 1;
}
```

### Substrate Types

Here is an example of how you would store substrate types AccountId, Balance and Hash:
```
#[ink::contract]
mod MyContract {

    // Our struct will use those default ink! types
    #[ink(storage)]
    pub struct MyContract {
        // Store some AccountId
        my_account: AccountId,
        // Store some Balance
        my_balance: Balance,
        // Store some Hash
        my_hash: Hash,
    }
    /* --snip-- */
}
```

### Enum 
Enum can be used as a datatype as well.

```
pub enum Status {
    /// An auction has not started yet.
    NotStarted,
    /// We are in the starting period of the auction, collecting initial bids.
    OpeningPeriod,
    /// We are in the ending period of the auction, where we are taking snapshots
    /// of the winning bids.
}
```
### Struct
You can even combine all the above mentioned types in a custom struct which you can then store in the contract's storage.

```
mod MyContract {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;


    pub struct Auction {
        /// Branded name of the auction event.
        name: String,
        /// Some hash identifying the auction subject.
        subject: Hash,
        /// Auction status.
        status: Status, // Enum: Usage shown in next section
        /// Candle auction can have no winner.
        /// If auction is finalized, that means that the winner is determined.
        finalized: bool,
        /// vector
        vector: Vec<u8>,
    }

    #[ink(storage)]
    pub struct MyContract {
        // Store Auctions in a vec
        auctions: Vec<Auction>,
    }
}
```

The values of an enum should be referenced as Status::OpeningPeriod.

--> For Getter and setters in Ink! smart contracts genericaly can be differentiate from each other
   with &self and &mut Self => Means setter mostly have mutable self element. 

--> require() statemnts equvalent for ink!     
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            // equvalent of require() statemnt from solidity; 
            assert!(init_value==true,"bool must start as true");
            Self { value: init_value }
        }
#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod simple_contract {
    use scale::{Decode, Encode};

    #[derive(Decode, Encode, Copy, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct AmmPool {
        pub token_0: u32,
        pub token_1: u32,
        pub token_0_amount: u32,
        pub token_1_amount: u32,
        pub total_supply: u32,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct SimpleContract {
        /// Stores a single `AmmPool` value on the storage.
        pool: AmmPool,
    }

    impl SimpleContract {
        #[ink(constructor)]
        pub fn new(token_0: u32, token_1: u32) -> Self {
            Self {
                pool: AmmPool {
                    token_0,
                    token_1,
                    token_0_amount: Default::default(),
                    token_1_amount: Default::default(),
                    total_supply: Default::default(),
                },
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(123, 456)
        }

        #[ink(message)]
        pub fn swap(&mut self) {
            todo!()
        }

        #[ink(message)]
        pub fn add_liquidity(&mut self) {
            todo!()
        }

        #[ink(message)]
        pub fn remove_liquidity(&mut self) {
            todo!()
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> AmmPool {
            self.pool
        }

        fn _mint(&mut self, _to: u32, amount: u32) {
            self.pool.total_supply += amount;
            todo!()
        }

        fn _burn(&mut self, _from: u32, amount: u32) {
            self.pool.total_supply -= amount;
            todo!()
        }
    }

    // /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    // /// module and test functions are marked with a `#[test]` attribute.
    // /// The below code is technically just normal Rust code.
    // #[cfg(test)]
    // mod tests {
    //     /// Imports all the definitions from the outer scope so we can use them here.
    //     use super::*;
    //
    //     /// We test if the default constructor does its job.
    //     #[ink::test]
    //     fn default_works() {
    //         let simple_contract = SimpleContract::default();
    //         assert_eq!(simple_contract.get(), false);
    //     }
    //
    //     /// We test a simple use case of our contract.
    //     #[ink::test]
    //     fn it_works() {
    //         let mut simple_contract = SimpleContract::new(false);
    //         assert_eq!(simple_contract.get(), false);
    //         simple_contract.flip();
    //         assert_eq!(simple_contract.get(), true);
    //     }
    // }
    //
    // /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    // ///
    // /// When running these you need to make sure that you:
    // /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    // /// - Are running a Substrate node which contains `pallet-contracts` in the background
    // #[cfg(all(test, feature = "e2e-tests"))]
    // mod e2e_tests {
    //     /// Imports all the definitions from the outer scope so we can use them here.
    //     use super::*;
    //
    //     /// A helper function used for calling contract messages.
    //     use ink_e2e::build_message;
    //
    //     /// The End-to-End test `Result` type.
    //     type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
    //
    //     /// We test that we can upload and instantiate the contract using its default constructor.
    //     #[ink_e2e::test]
    //     async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    //         // Given
    //         let constructor = SimpleContractRef::default();
    //
    //         // When
    //         let contract_account_id = client
    //             .instantiate("simple_contract", &ink_e2e::alice(), constructor, 0, None)
    //             .await
    //             .expect("instantiate failed")
    //             .account_id;
    //
    //         // Then
    //         let get = build_message::<SimpleContractRef>(contract_account_id.clone())
    //             .call(|simple_contract| simple_contract.get());
    //         let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
    //         assert!(matches!(get_result.return_value(), false));
    //
    //         Ok(())
    //     }
    //
    //     /// We test that we can read and write a value from the on-chain contract contract.
    //     #[ink_e2e::test]
    //     async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    //         // Given
    //         let constructor = SimpleContractRef::new(false);
    //         let contract_account_id = client
    //             .instantiate("simple_contract", &ink_e2e::bob(), constructor, 0, None)
    //             .await
    //             .expect("instantiate failed")
    //             .account_id;
    //
    //         let get = build_message::<SimpleContractRef>(contract_account_id.clone())
    //             .call(|simple_contract| simple_contract.get());
    //         let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
    //         assert!(matches!(get_result.return_value(), false));
    //
    //         // When
    //         let flip = build_message::<SimpleContractRef>(contract_account_id.clone())
    //             .call(|simple_contract| simple_contract.flip());
    //         let _flip_result = client
    //             .call(&ink_e2e::bob(), flip, 0, None)
    //             .await
    //             .expect("flip failed");
    //
    //         // Then
    //         let get = build_message::<SimpleContractRef>(contract_account_id.clone())
    //             .call(|simple_contract| simple_contract.get());
    //         let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
    //         assert!(matches!(get_result.return_value(), true));
    //
    //         Ok(())
    //     }
    // }
}

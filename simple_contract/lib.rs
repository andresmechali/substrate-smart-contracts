#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod simple_contract {
    use ink::storage::Mapping;
    use scale::{Decode, Encode};

    type TokenId = u32;

    #[derive(Decode, Encode, Copy, Clone, Debug)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct AmmPool {
        pub token_0: TokenId,
        pub token_1: TokenId,
    }

    #[ink(event)]
    pub struct Swapped {
        token_in: TokenId,
        token_out: TokenId,
        token_in_amount: Balance,
        token_out_amount: Balance,
        account: AccountId,
    }

    #[ink(event)]
    pub struct LiquidityAdded {
        tokens: (TokenId, TokenId),
        amounts: (Balance, Balance),
        account: AccountId,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct SimpleContract {
        /// Stores a single `AmmPool` value on the storage.
        pool: AmmPool,
        /// Supply of tokens
        reserves: Mapping<TokenId, Balance>,
        /// Balances for accounts
        balances: Mapping<(AccountId, TokenId), Balance>,
        /// Fees accumulated in the contract
        fees: Mapping<TokenId, Balance>,
    }

    impl SimpleContract {
        #[ink(constructor)]
        pub fn new(token_0: TokenId, token_1: TokenId) -> Self {
            Self {
                pool: AmmPool { token_0, token_1 },
                reserves: Mapping::default(),
                balances: Mapping::default(),
                fees: Mapping::default(),
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(0, 1)
        }

        #[ink(message)]
        /// Adds liquidity to the pool. Amount is equal for each token.
        pub fn add_liquidity(&mut self, amount: Balance) {
            let (token_0, token_1) = (self.pool.token_0, self.pool.token_1);

            // Update pool reserves
            let old_token_0_amount = self.reserves.get(token_0).unwrap_or_default();
            let new_token_0_amount = old_token_0_amount + amount;
            self.reserves.insert(token_0, &new_token_0_amount);
            let old_token_1_amount = self.reserves.get(token_1).unwrap_or_default();
            let new_token_1_amount = old_token_1_amount + amount;
            self.reserves.insert(token_1, &new_token_1_amount);

            // Update account's balances
            let account = self.env().caller();
            let old_token_0_balance = self.balances.get((account, token_0)).unwrap_or_default();
            let new_token_0_balance = old_token_0_balance + amount;
            self.balances
                .insert((account, token_0), &new_token_0_balance);
            let old_token_1_balance = self.balances.get((account, token_1)).unwrap_or_default();
            let new_token_1_balance = old_token_1_balance + amount;
            self.balances
                .insert((account, token_1), &new_token_1_balance);

            Self::env().emit_event(LiquidityAdded {
                tokens: (token_0, token_1),
                amounts: (amount, amount),
                account,
            })
        }

        #[ink(message)]
        pub fn swap(&mut self, token_in: TokenId, amount: Balance) -> Balance {
            // Check that the token is part of the pool
            assert!(
                token_in == self.pool.token_0 || token_in == self.pool.token_1,
                "Token {} does not belong to liquidity pool",
                token_in
            );

            // Set proper tokens and reserves for pool
            let (token_in, token_out) = if token_in == self.pool.token_0 {
                (self.pool.token_0, self.pool.token_1)
            } else {
                (self.pool.token_1, self.pool.token_0)
            };
            let reserve_in = self.reserves.get(token_in).unwrap_or_default();
            let reserve_out = self.reserves.get(token_out).unwrap_or_default();

            // Subtract 0.3% fee.
            let token_in_amount = amount * 997 / 1000;

            // Update fees in storage.
            let fee = amount - token_in_amount;
            let old_fee = self.reserves.get(token_in).unwrap_or_default();
            let new_fee = old_fee + fee;
            self.reserves.insert(token_in, &new_fee);

            // Calculate amount to send of token out (including 0.3% fee).

            let token_out_amount = if (reserve_in + token_in_amount) != 0 {
                (reserve_out * token_in_amount) / (reserve_in + token_in_amount)
            } else {
                0
            };

            let pool_reserve_out = self.reserves.get(token_out).unwrap_or_default();
            assert!(
                token_out_amount <= pool_reserve_out,
                "Pool does not have enough balance of token ({})",
                token_out
            );

            // Transfer amount of token_in to contract address.
            let new_reserve_in = reserve_in + token_in_amount;
            self.reserves.insert(token_in, &new_reserve_in);
            let old_balance_in = self
                .balances
                .get((self.env().caller(), token_in))
                .unwrap_or_default();
            let new_balance_in = old_balance_in + token_in_amount;
            self.balances
                .insert((self.env().caller(), token_in), &new_balance_in);

            // Transfer amount_out of token_out to account.
            let new_reserve_out = reserve_out - token_out_amount;
            self.reserves.insert(token_out, &new_reserve_out);
            let old_balance_out = self
                .balances
                .get((self.env().caller(), token_out))
                .unwrap_or_default();
            let new_balance_out = old_balance_out - token_out_amount;
            self.balances
                .insert((self.env().caller(), token_out), &new_balance_out);

            Self::env().emit_event(Swapped {
                token_in,
                token_in_amount,
                token_out,
                token_out_amount,
                account: self.env().caller(),
            });

            token_out_amount
        }

        #[ink(message)]
        pub fn remove_liquidity(&mut self) {
            // todo!()
        }

        /// Returns the current value of the pool's reserves.
        #[ink(message)]
        pub fn get_reserve(&self, token: TokenId) -> Balance {
            self.reserves.get(token).unwrap_or_default()
        }

        /// Returns the current value of account's balances for a given token.
        #[ink(message)]
        pub fn get_balance(&self, token: TokenId) -> Balance {
            self.balances
                .get((self.env().caller(), token))
                .unwrap_or_default()
        }

        /// Returns the total accumulated fees.
        #[ink(message)]
        pub fn get_fees(&self, token: TokenId) -> Balance {
            self.fees.get(token).unwrap_or_default()
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

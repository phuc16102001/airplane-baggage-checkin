#[cfg(test)]
mod tests {
    use near_sdk::{MockedBlockchain};
    use near_sdk::{testing_env, VMContext};

    fn get_context_airline(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "dev-account.testnet".to_string(),
            signer_account_id: "phuc16102001.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "phuc16102001.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 1000000000000000000000000,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    fn get_context_customer(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "dev-account.testnet".to_string(),
            signer_account_id: "thanhhoang4869.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "thanhhoang4869.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 1000000000000000000000000,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }
    
    #[test]
    fn init_contract() {
        let context = get_context_airline(vec![], false);
        testing_env!(context);

        let mut contract = Contract::new();
        assert_eq!(
            contract.get_initialized(),
            false
        );
        
        contract.init("phuc16102001.testnet".to_string());
        assert_eq!(
            contract.get_payment_account(),
            "phuc16102001.testnet"
        );
        assert_eq!(
            contract.get_owner(),
            "phuc16102001.testnet"
        );
        assert_eq!(
            contract.get_initialized(),
            true
        );
    }

    #[test]
    fn reset_payment_account() {
        let context = get_context_airline(vec![], false);
        testing_env!(context);
        let mut contract = Contract::new();

        contract.init("phuc16102001.testnet".to_string());
        contract.reset();
        contract.init("thanhhoang4869.testnet".to_string());
    
        assert_eq!(
            contract.get_payment_account(),
            "thanhhoang4869.testnet"
        );
        assert_eq!(
            contract.get_owner(),
            "phuc16102001.testnet"
        );
    }

    #[test]
    fn registry_flight() {
        let context_airline = get_context_airline(vec![], false);
        let context_customer = get_context_customer(vec![], false);

        testing_env!(context_airline);
        let mut contract = Contract::new();
        contract.init("phuc16102001.testnet".to_string());
    
        testing_env!(context_customer);
        contract.registry(1, FlightClass::First, 10.0);
    }
}
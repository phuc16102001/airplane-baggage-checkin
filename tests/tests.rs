#[cfg(test)]
mod tests {
    use airplane_baggage_checking::*;
    use airplane_baggage_checking::flight_detail::*;
    use airplane_baggage_checking::types::*;
    use near_sdk::{MockedBlockchain, Balance};
    use near_sdk::{testing_env, VMContext};

    fn get_context(
        input: Vec<u8>, 
        is_view: bool, 
        predecessor_account_id: String,
        attached_deposit: Balance
    ) -> VMContext {
        VMContext {
            current_account_id: "dev&-account.testnet".to_string(),
            signer_account_id: predecessor_account_id.clone(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: predecessor_account_id.clone(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 10u128.pow(32),
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: attached_deposit,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    fn get_context_airlines() -> VMContext {
        get_context(
            vec![], 
            false, 
            "phuc16102001.testnet".to_string(), 
            0
        )
    }

    fn get_context_customer(attached_deposit: Balance) -> VMContext {
        get_context(
            vec![], 
            false, 
            "thanhhoang4869.testnet".to_string(),
            attached_deposit
        )
    }
    
    #[test]
    fn init_contract() {
        let context = get_context_airlines();
        testing_env!(context);

        let mut contract = Contract::default();
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
    #[should_panic(
        expected = r#"Contract was already initialized"#
    )]
    fn double_init() {
        let context = get_context_airlines();
        testing_env!(context);

        let mut contract = Contract::default();
        assert_eq!(
            contract.get_initialized(),
            false
        );
        
        contract.init("phuc16102001.testnet".to_string());
        contract.init("phuc16102001.testnet".to_string());
    }

    #[test]
    fn reset_payment_account() {
        let context = get_context_airlines();
        testing_env!(context);
        let mut contract = Contract::default();

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
        let context_airline = get_context_airlines();
        let context_customer = get_context_customer(0);

        testing_env!(context_airline);
        let mut contract = Contract::default();
        contract.init("phuc16102001.testnet".to_string());
    
        testing_env!(context_customer);
        let flight_id = 1;
        contract.registry(1, FlightClass::First, 10.0);
        assert_eq!(
            contract.check_class(flight_id),
            "First"
        );
    }

    #[test]
    #[should_panic(
        expected=r#"Cannot find your flight"#
    )]
    fn add_baggages_fail() {
        let context_airline = get_context_airlines();
        let context_customer = get_context_customer(0);

        testing_env!(context_airline);
        let mut contract = Contract::default();
        contract.init("phuc16102001.testnet".to_string());
    
        testing_env!(context_customer);
        let flight_id = 1;
        contract.add_baggage(flight_id, 4.0);
    }

    #[test]
    fn add_baggages_success() {
        let context_airline = get_context_airlines();
        let context_customer = get_context_customer(0);

        testing_env!(context_airline);
        let mut contract = Contract::default();
        contract.init("phuc16102001.testnet".to_string());
    
        testing_env!(context_customer);
        let flight_id = 1;
        contract.registry(1, FlightClass::First, 10.0);
        let baggage_id = contract.add_baggage(flight_id, 4.0);
        let baggage = contract.check_baggage(flight_id, baggage_id);
        
        assert_eq!(
            baggage_id,
            0
        );
        assert_eq!(
            *baggage.get_id(),
            0
        );
        assert_eq!(
            *baggage.get_weight(),
            4.0
        );
    }

    #[test]
    fn count_baggages() {
        let context_airline = get_context_airlines();
        let context_customer = get_context_customer(0);

        testing_env!(context_airline);
        let mut contract = Contract::default();
        contract.init("phuc16102001.testnet".to_string());
    
        testing_env!(context_customer);
        let flight_id = 1;
        contract.registry(1, FlightClass::First, 10.0);
        contract.add_baggage(flight_id, 4.0);
        contract.add_baggage(flight_id, 1.0);
        
        assert_eq!(
            contract.check_number_baggages(flight_id),
            2
        );
    }

    #[test]
    #[should_panic(
        expected=r#"You cannot add more than 3 baggages"#
    )]
    fn limit_baggages() {
        let context_airline = get_context_airlines();
        let context_customer = get_context_customer(0);

        testing_env!(context_airline);
        let mut contract = Contract::default();
        contract.init("phuc16102001.testnet".to_string());
    
        testing_env!(context_customer);
        let flight_id = 1;
        contract.registry(1, FlightClass::First, 10.0);
        contract.add_baggage(flight_id, 1.0);
        contract.add_baggage(flight_id, 3.0);
        contract.add_baggage(flight_id, 5.0);
        contract.add_baggage(flight_id, 4.0);
    }

    #[test]
    #[should_panic(
        expected=r#"Cannot find your baggage"#
    )]
    fn not_found_baggages() {
        let context_airline = get_context_airlines();
        let context_customer = get_context_customer(0);

        testing_env!(context_airline);
        let mut contract = Contract::default();
        contract.init("phuc16102001.testnet".to_string());
    
        testing_env!(context_customer);
        let flight_id = 1;
        contract.registry(1, FlightClass::First, 10.0);
        
        let baggage_id = contract.add_baggage(flight_id, 1.0);
        let other_id = baggage_id+1;
        contract.check_baggage(flight_id, other_id);
    }

    fn check_fee(
        flight_class: FlightClass, 
        distance: Distance,
        first_weight: Weight,
        second_weight: Weight,
        third_weight: Weight,
        expected_price: Balance
    ) {
        let context_airline = get_context_airlines();
        let context_customer = get_context_customer(0);

        testing_env!(context_airline);
        let mut contract = Contract::default();
        contract.init("phuc16102001.testnet".to_string());
    
        testing_env!(context_customer);
        let flight_id = 1;
        contract.registry(flight_id, flight_class, distance);
        
        contract.add_baggage(flight_id, first_weight);
        contract.add_baggage(flight_id, second_weight);
        contract.add_baggage(flight_id, third_weight);

        assert_eq!(
            contract.check_fee(flight_id),
            expected_price
        )
    }

    #[test]
    fn check_fee_first() {
        check_fee(
            FlightClass::First,
            100.0,
            5.0, 2.0, 3.0,
            10
        )
    }

    #[test]
    fn check_fee_business() {
        check_fee(
            FlightClass::Business,
            100.0,
            5.0, 2.0, 3.0,
            20
        )
    }

    #[test]
    fn check_fee_economy() {
        check_fee(
            FlightClass::Economy,
            100.0,
            5.0, 2.0, 3.0,
            220
        )
    }
}
# Introduction - Airline baggage checking

## Overview

This is a contract designed for the airline baggage checking. It allows the customers to deposit their money and let the airline to delivery their baggages. In the real situation, this may be a complicated process and risky. With smart contract, the process works faster and safer.

## Contract flow

In this workflow, we have three different roles:
- `Owner`: The one who create contract
- `Airline`: The airline company
- `Customer`: The user who want to deposit 

The contract has several steps:
- The contract first initialized by the `owner` and set payment to `airline`
- `Customer` registry their flight (with `flight_id`, `flight_class` and `distance`)
- For each flight, they can check at most 3 baggages (with `baggage_size`)
- The system print out the `baggage_id` after checking
- Check for the `price` 
- Accept the `fee` and deposit money to contract
- Baggages now are delivering
- Finally, the `Customer` claim baggages and the contract will send the money to `airline`

> Moreover, `Customer` can check many different attributes such as `flight_class`, their checked `baggages`, etc

## Rules
- Only `owner` can `reset` the contract
- `Customer` can only check their baggages when they were `registered`
- There are three different `flight_classes`, each one has a different `fee` (in NEAR unit) strategy:
  - `First class`: 2*(total weight, except the heaviest one)
  - `Business class`: 2*(total weight)
  - `Economy class`: 2*(total weight + distance)
- Each `Customer` in a `flight`, can only have `at most 3 baggages`
- Only the `Airline` can call the `deliver_baggage` method
- Every operations must follow the workflow (e.g. claim can only operate after delivering)

# Folder structure
- `baggage.rs`: The structure of a baggage
- `fee.rs`: The fee strategy classes (using strategy design pattern)
- `flight_detail.rs`: The structure of a flight detail (for each customer)
- `types.rs`: Definition of data types
- `lib.rs`: The main source code of my contract
- `test.rs`: Unit test source

# Unit test
```Bash
cargo test
```

# Manual test
## Compile and build
```Bash
./build.sh
```

## Dev deploy
```Bash
./dev-deploy.sh
```

## Create variables
```Bash
source 'neardev/dev-account.env'
export OWNER='phuc16102001.testnet'
export AIRLINE='phuc16102001.testnet'
export CUSTOMER='thanhhoang4869.testnet'
```

## Init contract
```Bash
near call $CONTRACT_NAME init '{"payment_account":"'$AIRLINE'"}' --accountId $OWNER
```

## Registry flight
```Bash
near call $CONTRACT_NAME registry '{"flight_id":1, "flight_class":"First", "distance": 10}' --accountId $CUSTOMER
```

## Baggage operations

### Add new baggage
```Bash
near call $CONTRACT_NAME add_baggage '{"flight_id":1,"baggage_weight":4}' --accountId $CUSTOMER
```

### Remove baggage
```Bash
near call $CONTRACT_NAME remove_baggage '{"flight_id":1,"baggage_id":0}' --accountId $CUSTOMER
```

### Remove all baggages
```Bash
near call $CONTRACT_NAME remove_all_baggages '{"flight_id":1}' --accountId $CUSTOMER
```

## Check status operations

### Check flight state
```Bash
near call $CONTRACT_NAME check_state '{"flight_id":1}' --accountId $CUSTOMER
```

### Check flight class
```Bash
near call $CONTRACT_NAME check_class '{"flight_id":1}' --accountId $CUSTOMER
```

### Check the number of current baggages
```Bash
near call $CONTRACT_NAME check_number_baggages '{"flight_id":1}' --accountId $CUSTOMER
```

### Check a baggage
```Bash
near call $CONTRACT_NAME check_baggage '{"flight_id":1, "baggage_id":0}' --accountId $CUSTOMER
```

### Check fee before depositing
```Bash
near call $CONTRACT_NAME check_fee '{"flight_id":1}' --accountId $CUSTOMER
```

## Process operations

### Deposit (accept)
```Bash
near call $CONTRACT_NAME accept '{"flight_id":1}' --accountId $CUSTOMER --amount 10
```

### Delivering baggages
```Bash
near call $CONTRACT_NAME deliver_baggage '{"flight_id":1, "customer_id": "'$CUSTOMER'"}' --accountId $AIRLINE
```

### Claim baggages
```Bash
near call $CONTRACT_NAME claim_baggages '{"flight_id":1}' --accountId $CUSTOMER
```

# Contribution
This project was implemented by [phuc16102001](https://github.com/phuc16102001)

You can reference from this, but **do not copy**

These are the project which I have referenced from:
- [Tin Writes Code](https://github.com/tinwritescode)
- [Sample lottery](https://github.com/Learn-NEAR/NCD.L1.sample--lottery)
- [Meme museum](https://github.com/Learn-NEAR/NCD.L1.sample--meme-museum)
- [Cold chain delivery](https://github.com/Learn-NEAR/NCD.L1.sample--cold-chain-delivery)
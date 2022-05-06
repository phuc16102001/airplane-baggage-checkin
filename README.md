# Introduction - Airline baggage checking


# Compile and build
```Bash
./build.sh
```

# Unit test

# Manual test
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

## Add new baggage
```Bash
near call $CONTRACT_NAME add_baggage '{"flight_id":1,"baggage_weight":4}' --accountId $CUSTOMER
```

## Remove baggage
```Bash
near call $CONTRACT_NAME remove_baggage '{"flight_id":1,"baggage_id":0}' --accountId $CUSTOMER
```

## Remove all baggages
```Bash
near call $CONTRACT_NAME remove_all_baggages '{"flight_id":1}' --accountId $CUSTOMER
```

## Check current baggages
```Bash
near call $CONTRACT_NAME check_baggages '{"flight_id":1}' --accountId $CUSTOMER
```

## Check price for deposit
```Bash
near call $CONTRACT_NAME check_price '{"flight_id":1}' --accountId $CUSTOMER
```

## Deposit (accept)
```Bash
near call $CONTRACT_NAME accept '{"flight_id":1}' --accountId $CUSTOMER --amount 10
```

## Transporting baggages
```Bash
near call $CONTRACT_NAME transport_baggage '{"flight_id":1}' --accountId $CUSTOMER
```

## Claim baggages
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
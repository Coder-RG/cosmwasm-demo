# Cosmwasm demo

Design of smart contract is as follows:

> A person can upload a contract with some funding requirement(similar to crowdfunding).
> Anyone can send the money to fulfill this contract.
> If the tokens are less than the required amount, it fails.
> Else the money is forwarded to the 'owner'/'recipient'.


```sh
$ source <(curl -sSL https://raw.githubusercontent.com/CosmWasm/testnets/master/malaga-420/defaults.env)

$ wasmd keys add wallet

$ wasmd keys add wallet2

$ JSON=$(jq -n --arg addr $(wasmd keys show -a wallet) '{"denom":"umlg","address":$addr}') && curl -X POST --header "Content-Type: application/json" --data "$JSON" https://faucet.malaga-420.cosmwasm.com/credit

$ JSON=$(jq -n --arg addr $(wasmd keys show -a wallet2) '{"denom":"umlg","address":$addr}') && curl -X POST --header "Content-Type: application/json" --data "$JSON" https://faucet.malaga-420.cosmwasm.com/credit

$ export NODE=(--node $RPC)
$ export TXFLAG=($NODE --chain-id $CHAIN_ID --gas-prices 0.25umlg --gas auto --gas-adjustment 1.3)

$ RES=$(wasmd tx wasm store artifacts/simple-test-case-aarch64.wasm --from wallet $TXFLAG -y --output json -b block)

$ CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value')

$ wasmd query wasm list-contract-by-code $CODE_ID $NODE --output json

$ wasmd query wasm code $CODE_ID $NODE download.wasm

$ diff artifacts/cw_nameservice.wasm download.wasm

$ INIT='{"capital":100,"end_height":266000}'

$ wasmd tx wasm instantiate $CODE_ID "$INIT" --from wallet --label "First contract" $TXFLAG -y --no-admin
```
>Error: rpc error: code = InvalidArgument desc = failed to execute message; message index: 0: Error parsing into type simple_test_case::msg::InstantiateMsg: Invalid type: instantiate wasm contract failed: invalid request
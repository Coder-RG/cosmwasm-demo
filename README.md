# Cosmwasm demo

Design of smart contract is as follows:

> A person can upload a contract with some funding requirement(similar to crowdfunding).
> Anyone can send the money to fulfill this contract.
> If the tokens are less than the required amount, it fails.
> Else the money is forwarded to the 'owner'/'recipient'.

This is a template to build smart contracts in Rust to run inside a
[Cosmos SDK](https://github.com/cosmos/cosmos-sdk) module on all chains that enable it.
To understand the framework better, please read the overview in the
[cosmwasm repo](https://github.com/CosmWasm/cosmwasm/blob/master/README.md),
and dig into the [cosmwasm docs](https://www.cosmwasm.com).
This assumes you understand the theory and just want to get coding.

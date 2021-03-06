#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{config, config_read, State};

// version info for migration info
// const CONTRACT_NAME: &str = "crates.io:simple-test-case";
// const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender,
        sender: None,
        capital: msg.capital,
        end_height: msg.end_height,
        // end_time: msg.end_time,
    };

    if state.is_expired(&env) {
        return Err(ContractError::Expired {
            end_height: msg.end_height,
            // end_time: msg.end_time,
        });
    }

    config(deps.storage).save(&state)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let state = config(deps.storage).load()?;
    match msg {
        ExecuteMsg::Transfer {} => handle_transfer(deps, env, state, info),
        // ExecuteMsg::Refund {} => handle_refund(deps, env, state, info),
    }
}

pub fn handle_transfer(
    _deps: DepsMut,
    env: Env,
    mut state: State,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // Check if the contract has expired
    if state.is_expired(&env) {
        return Err(ContractError::Expired {
            end_height: state.end_height,
            // end_time: state.end_time,
        });
    }
    // If the amount is less than the capital being raised,
    // show an error
    if info.funds[0].amount.u128() != state.capital {
        return Err(ContractError::InsufficientFunds {
            funds: state.capital,
            sent: info.funds,
        });
    }

    // Else proceed with the transfer
    state.sender = Some(info.sender);
    Ok(Response::new()
        .add_message(BankMsg::Send {
            to_address: state.owner.to_string(),
            amount: info.funds,
        })
        .add_attribute("action", "transfer"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&config_read(deps.storage).load()?),
    }
}

pub fn query_config(deps: Deps) -> State {
    let state = config_read(deps.storage).load().unwrap();
    state
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, Addr, BankMsg, CosmosMsg, Timestamp, Uint128};

    fn init_msg_expire_by_height(height: u64) -> InstantiateMsg {
        InstantiateMsg {
            capital: 200u128,
            end_height: Some(height),
        }
    }

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let msg = init_msg_expire_by_height(5);
        let mut env = mock_env();
        env.block.height = 3;
        env.block.time = Timestamp::from_seconds(0);

        let info = mock_info("recipient", &coins(200, "upebble"));
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let res = query_config(deps.as_ref());
        assert_eq!(200u128, res.capital);
        assert_eq!(Addr::unchecked("recipient"), res.owner);
        assert_eq!(None, res.sender);
        assert_eq!(Some(5), res.end_height);
    }

    #[test]
    fn expired_contract_before_init() {
        let mut deps = mock_dependencies();
        let msg = init_msg_expire_by_height(5);
        let mut env = mock_env();
        env.block.height = 6;
        env.block.time = Timestamp::from_seconds(0);

        let info = mock_info("sender", &coins(200, "upebble"));
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap_err();
        match res {
            ContractError::Expired { .. } => {}
            e => panic!("Another error occured {:?}", e),
        }
    }

    #[test]
    fn expired_contract_after_init() {
        let mut deps = mock_dependencies();
        let msg = init_msg_expire_by_height(5);
        let mut env = mock_env();
        env.block.height = 4;
        env.block.time = Timestamp::from_seconds(0);

        // Instantiate without any issues
        let info = mock_info("sender", &coins(200, "upebble"));
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        // Move block height to anything greater than 5 to validate expiry
        env.block.height = 6;
        env.block.time = Timestamp::from_seconds(4);

        let msg = ExecuteMsg::Transfer {};
        let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
        match err {
            ContractError::Expired { .. } => {}
            e => panic!("Another error occured: {:?}", e),
        }
    }

    #[test]
    fn transfer_insufficient_funds() {
        let mut deps = mock_dependencies();
        let msg = init_msg_expire_by_height(5);
        let mut env = mock_env();
        env.block.height = 3;
        env.block.time = Timestamp::from_seconds(0);

        // This should be 200upebbles, not 100upebbles
        let info = mock_info("sender", &coins(100, "upebble"));
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        let msg = ExecuteMsg::Transfer {};
        let res = execute(deps.as_mut(), env, info, msg).unwrap_err();
        match res {
            ContractError::InsufficientFunds { .. } => {}
            e => panic!("Another error occured: {:?}", e),
        }
    }

    #[test]
    fn tranfer_sufficient_funds() {
        let mut deps = mock_dependencies();
        let msg = init_msg_expire_by_height(5);
        let mut env = mock_env();
        env.block.height = 3;
        env.block.time = Timestamp::from_seconds(0);

        let info = mock_info("recipient", &coins(0, "upebble"));
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        let info = mock_info("sender", &coins(200, "upebble"));
        let msg = ExecuteMsg::Transfer {};
        let res = execute(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(1, res.messages.len());
        assert_eq!(
            res.messages[0].msg,
            CosmosMsg::Bank(BankMsg::Send {
                to_address: "recipient".into(),
                amount: coins(200, "upebble"),
            })
        );
    }
    #[test]
    fn number_of_coins() {
        let value = coins(12, "upebble");
        assert_eq!(1, value.len());
        assert_eq!(Uint128::from(12u64).u128(), value[0].amount.u128());
    }
}



use core::f32;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr};
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:transfer";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    if msg.expiration <= cw_utils::Expiration::AtHeight(_env.block.height) {
        return Err(ContractError::Unauthorized  {});
      }

    let state = State {
        owner: info.sender.clone(),
        source: None,
        sent_coins: None,
        beneficiary_1:  None,
        beneficiary_2: None,
        beneficiary1_balance: None,
        beneficiary2_balance: None, 
        expiration: msg.expiration,

    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        
    )
}


#[cfg_attr(not(feature = "library"), {sentCoins:f32,beneficiary1:Addr,beneficiary2:Addr, amount }
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SendCoins { sentCoins: f32 , beneficiary1: Addr, beneficiary2: Addr} => execute::send_coins(deps, sentCoins, beneficiary1, beneficiary2 ),
        ExecuteMsg::WithdrawCoins { fromAccount: String, quantity: Vec<Coin>} => execute::withdraw_coins(deps, _env, info, fromAccount, quantity),

       
    }
}

pub mod execute {
    use cosmwasm_std::{Coin};

    use super::*;

    pub fn send_coins(deps: DepsMut, sentCoins : f32) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.beneficiary1_balance += Vec::<Coin> (sentCoins/2.0);
            state.beneficiary2_balance += Vec::<Coin> (sentCoins/2.0);
            Ok(state)
        })?;

        Ok(Response::new().add_attribute("action", "send_coins"))
    }

    pub fn withdraw_coins(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
                
            }

        
            //state.count = count;
            //Ok(state)
            Ok(Response::new().add_attribute("action", "withdraw_coins"))
        })?;
        Ok(Response::new().add_attribute("action", "reset"))
    }
}



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg, account: Option<Addr>) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner{}=>to_binary(&query::owner(deps)?),
        QueryMsg::GetBalance{account}=>to_binary(&query::balance(deps,Some(account))?),
        QueryMsg::GetStateResponse {  } => to_binary(&query::state(deps)?), }
}

pub mod query {

    use crate::msg::{OwnerResponse, BalanceResponse, ConfigResponse};

    use super::*;

    pub fn owner(deps: Deps) -> StdResult<OwnerResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(OwnerResponse { owner: state.owner })
    }

    pub fn balance (deps: Deps, account: Option<Addr>) -> StdResult<BalanceResponse> {
        let state = STATE.load(deps.storage)?;
        let balance1 = state.beneficiary1_balance;
        let balance2 = state.beneficiary2_balance;

        if account == state.beneficiary_1{
            Ok(BalanceResponse { balance: state.beneficiary1_balance,})
         } else if account == state.beneficiary_2{
            Ok(BalanceResponse { balance: state.beneficiary2_balance,})
         } else{
            Ok(BalanceResponse { balance: None,})
         }
    }
    pub fn state(deps: Deps) -> StdResult<ConfigResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(state)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { expiration: cw_utils::Expiration::AtHeight(100000) };
        let info = mock_info("account", &coins(1000, "earth"));
        let owner = info.sender;
        let balance = Some(info.funds);

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOwner {}, Some(owner)).unwrap();
        let value: crate::msg::OwnerResponse = from_binary(&res).unwrap();
        assert_eq!(owner, value.owner);

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance { account: (owner)}, Some(owner) ).unwrap();
        let value: crate::msg::BalanceResponse = from_binary(&res).unwrap();
        assert_eq!(balance, value.balance);
    }

    #[test]
    fn send_coins() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { expiration: cw_utils::Expiration::AtHeight(100000) };
        let info = mock_info("creator", &coins(2, "token"));
        let info1 = mock_info("creator", &coins(2, "token"));
        let info2 = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        //let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::SendCoins { amount: (Some(info.funds)), beneficiary1: (info1.sender), beneficiary2: (info2.sender) } ;
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance { account: info.sender } , Some(info1.sender)).unwrap();
        let value: crate::msg::BalanceResponse = from_binary(&res).unwrap();
        assert_eq!(Some(info.funds), value.balance);
    }

    #[test]
    fn withdraw_coins() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { expiration: cw_utils::Expiration::AtHeight(100000) };
        let info = mock_info("creator", &coins(2, "token"));
        let info1 = mock_info("creator", &coins(2, "token"));
        let info2 = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // withdraw coins then check balance
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::WithdrawCoins { account: (info.sender), quantity: (Some(info.funds)) };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::WithdrawCoins { account: (auth_info.sender), quantity: (Some(auth_info.funds)) } ;
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance { account: (auth_info.sender) }, Some(info2.sender) ).unwrap();
        let value: crate::msg::BalanceResponse = from_binary(&res).unwrap();
        assert_eq!(Some(auth_info.funds), value.balance);
    }
}

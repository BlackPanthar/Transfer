#[cfg(not(feature = "library"))]
use cosmwasm_std::{to_binary, entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr};
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};
use cosmwasm_std::StdError;
//use core::f32;

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
        //return Err(ContractError::Unauthorized  {});
        return Err(ContractError::Std(StdError::generic_err("Cannot create expired contract")));
      }

    let state = State {
        owner: info.sender.clone(), 
        source: "".to_string(),
        sent_coins: 0,
        beneficiary_1:  "".to_string(),
        beneficiary_2: "".to_string(),
        beneficiary1_balance: 0,
        beneficiary2_balance: 0, 
        expiration: msg.expiration,

    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("expiration", msg.expiration.to_string())
        
    )
}


#[cfg_attr(not(feature = "library"), entry_point)]
//#[cfg_attr(not(feature = "library"), sentCoins:<Option<Vec<Coin>>,beneficiary1:Addr,beneficiary2:Addr, amount )]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SendCoins { 
            sentCoins, 
            beneficiary1,
            beneficiary2} 
            => execute::send_coins(deps, info, sentCoins, beneficiary1, beneficiary2 ),
        ExecuteMsg::WithdrawCoins { 
            fromAccount,
            toWithdraw} 
            => execute::withdraw_coins(deps, info, fromAccount, toWithdraw),
    }
}

pub mod execute {
    use super::*;

    pub fn send_coins(deps: DepsMut, exec_info: MessageInfo, sentCoins : i32, beneficiary1: String, 
        beneficiary2: String) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        
            state.source = exec_info.sender.clone().to_string();
            state.sent_coins = sentCoins;
            state.beneficiary_1 = beneficiary1;
            state.beneficiary_2 = beneficiary2;
            
            state.beneficiary1_balance += sentCoins/2;
            state.beneficiary2_balance += sentCoins/2;

            Ok(state)
        })?;



        Ok(Response::new().add_attribute("method", "send_coins"))
    }

    pub fn withdraw_coins(deps: DepsMut, info: MessageInfo, fromAccount: String, toWithdraw: i32) 
    -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if fromAccount == state.beneficiary_1.to_string() {
                if state.beneficiary1_balance <= 0{
                    state.beneficiary_1 = "".to_string();
                    return Err(ContractError::Std(StdError::generic_err("Withdrawal limit reached")));
                }
                state.beneficiary1_balance -= toWithdraw;
            }
            else if fromAccount == state.beneficiary_2.to_string(){
                if state.beneficiary2_balance <= 0{
                    state.beneficiary_2 = "".to_string();
                    return Err(ContractError::Std(StdError::generic_err("Withdrawal limit reached")));
                }
                state.beneficiary2_balance -= toWithdraw;
            } else{
                return Err(ContractError::Unauthorized {});
            }
            Ok(state)

        })?;

        Ok(Response::new().add_attribute("method", "withdraw_coins"))
    }
}



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg, account: String) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner{}=>to_binary(&query::owner(deps)?),
        QueryMsg::GetBalance{account}=>to_binary(&query::balance(deps,account)?),
         }
}

pub mod query {

    use crate::msg::{OwnerResponse, BalanceResponse, ConfigResponse};

    use super::*;

    pub fn owner(deps: Deps) -> StdResult<OwnerResponse> {
        let state = STATE.load(deps.storage)?;
        let _owner = state.owner;
        Ok(OwnerResponse { owner: _owner ,})
    }

    pub fn balance (deps: Deps, account: String) -> StdResult<BalanceResponse> {
        let state = STATE.load(deps.storage)?;

        if account == state.beneficiary_1{
            Ok(BalanceResponse { balance: state.beneficiary1_balance,})
         } else if account == state.beneficiary_2{
            Ok(BalanceResponse { balance: state.beneficiary2_balance,})
         } else{
            Ok(BalanceResponse { balance: 0,})
         }
    }
/* *   pub fn state(deps: Deps) -> StdResult<ConfigResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(state)
    }*/
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
        let msg = ExecuteMsg::SendCoins { sentCoins: (info.funds), beneficiary1: (info1.sender), beneficiary2: (info2.sender) } ;
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
        let msg = ExecuteMsg::WithdrawCoins { fromAccount: (info.sender), toWithdraw: (Some(info.funds)) };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::WithdrawCoins { fromAccount: (auth_info.sender), toWithdraw: (Some(auth_info.funds)) } ;
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance { account: (auth_info.sender) }, Some(info2.sender) ).unwrap();
        let value: crate::msg::BalanceResponse = from_binary(&res).unwrap();
        assert_eq!(Some(auth_info.funds), value.balance);
    }
}

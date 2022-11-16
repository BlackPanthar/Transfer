#[cfg(not(feature = "library"))]
use cosmwasm_std::{to_binary, entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};
use cosmwasm_std::StdError;

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
            sent_coins, 
            beneficiary1,
            beneficiary2} 
            => execute::send_coins(deps, info, sent_coins, beneficiary1, beneficiary2 ),
        ExecuteMsg::WithdrawCoins { 
            from_account,
            to_withdraw} 
            => execute::withdraw_coins(deps, info, from_account, to_withdraw),
    }
}

pub mod execute {
    use super::*;

    pub fn send_coins(deps: DepsMut, exec_info: MessageInfo, sent_coins : i32, beneficiary1: String, 
        beneficiary2: String) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        
            state.source = exec_info.sender.clone().to_string();
            state.sent_coins = sent_coins;
            state.beneficiary_1 = beneficiary1;
            state.beneficiary_2 = beneficiary2;
            
            state.beneficiary1_balance += sent_coins/2;
            state.beneficiary2_balance += sent_coins/2;

            Ok(state)
        })?;



        Ok(Response::new().add_attribute("method", "send_coins"))
    }

    pub fn withdraw_coins(deps: DepsMut, _info: MessageInfo, from_account: String, to_withdraw: i32) 
    -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if from_account == state.beneficiary_1.to_string() {
                if state.beneficiary1_balance <= 0{
                    state.beneficiary_1 = "".to_string();
                    return Err(ContractError::Std(StdError::generic_err("Withdrawal limit reached")));
                }
                state.beneficiary1_balance -= to_withdraw;
            }
            else if from_account == state.beneficiary_2.to_string(){
                if state.beneficiary2_balance <= 0{
                    state.beneficiary_2 = "".to_string();
                    return Err(ContractError::Std(StdError::generic_err("Withdrawal limit reached")));
                }
                state.beneficiary2_balance -= to_withdraw;
            } else{
                return Err(ContractError::Unauthorized {});
            }
            Ok(state)

        })?;

        Ok(Response::new().add_attribute("method", "withdraw_coins"))
    }
}



//#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg, _from_account: String) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner{}=>to_binary(&query::owner(deps)?),
        QueryMsg::GetBalance{from_account}=>to_binary(&query::balance(deps,from_account)?),
         }
}

pub mod query {

    use crate::msg::{OwnerResponse, BalanceResponse};

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
        

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOwner {}, owner.into_string()).unwrap();
        let value: crate::msg::OwnerResponse = from_binary(&res).unwrap();
        assert_eq!(owner, value.owner);

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance { from_account: owner.into_string()}, owner.into_string()).unwrap();
        let value: crate::msg::BalanceResponse = from_binary(&res).unwrap();
        assert_eq!(1000, value.balance); 
    }

    #[test]
    fn send_coins() {
        //test that each account has 50% of the sent funds
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { expiration: cw_utils::Expiration::AtHeight(100000) }; 
        let info = mock_info("creator", &coins(2, "token"));
        let info1 = mock_info("beneficiary1", &coins(0, "token"));
        let info2 = mock_info("beneficiary2", &coins(0, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

   

        let msg = ExecuteMsg::SendCoins { 
            sent_coins: 100, 
            beneficiary1: info1.sender.into_string(), 
            beneficiary2: info2.sender.into_string() } ;
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // balance should be 50% of sent amount
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance { from_account: info1.sender.into_string() } , info1.sender.into_string()).unwrap();
        let value: crate::msg::BalanceResponse = from_binary(&res).unwrap();
        assert_eq!(50, value.balance);

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance { from_account: info2.sender.into_string() } , info2.sender.into_string()).unwrap();
        let value: crate::msg::BalanceResponse = from_binary(&res).unwrap();
        assert_eq!(50, value.balance);
    }
}
    
/* 
    #[test]
    fn withdraw_coins() {
      
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { expiration: cw_utils::Expiration::AtHeight(100000) };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();


        let unauth_info = mock_info("anyone", &coins(2, "token"));
        //let balance = unauth_info.funds.pop().amount;
        let msg = ExecuteMsg::WithdrawCoins { from_account: info.sender.into_string(), to_withdraw: 2 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        assert_eq!(10, res);


        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::WithdrawCoins { from_account: auth_info.sender.into_string(), to_withdraw: auth_info.funds } ;
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance { account: (auth_info.sender) }, Some(info2.sender) ).unwrap();
        let value: crate::msg::BalanceResponse = from_binary(&res).unwrap();
        assert_eq!(Some(auth_info.funds), value.balance);
    }*/
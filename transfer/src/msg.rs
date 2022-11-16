use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_utils::Expiration;
use crate::{state::State};

#[cw_serde]
pub struct InstantiateMsg {
    //owner comes from the MessageInfo/who signed it on the cli on instantation

    pub expiration: Expiration,
}


#[cw_serde]
pub enum ExecuteMsg {
    ///SendCoins: Account can send funds to contract and specify two beneficiary accounts
    /// WithdrawCoins: Beneficiary account can withdraw funds from its balance
    SendCoins {sentCoins: i32, beneficiary1: String, beneficiary2: String},
    WithdrawCoins {fromAccount: String, toWithdraw: i32}, 
}

#[cw_serde]
//#[derive(QueryResponses)]
pub enum QueryMsg {
   
 //   #[returns(ConfigResponse)]
   //   GetStateResponse {},


    //GetOwner returns the current owner as a json-encoded string
    //#[returns(ConfigResponse)]
       GetOwner {},

    //#[returns(ConfigResponse)]
    GetBalance { account : String,},
}

// We define a custom struct for each query response

pub type ConfigResponse = State;

#[cw_serde]
pub struct OwnerResponse {
    pub owner: Addr,
}

#[cw_serde]
pub struct BalanceResponse {
    pub balance: i32, 
}


use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_utils::Expiration;

use crate::{state::State};

#[cw_serde]
pub struct InstantiateMsg {
    //owner comes from the environment/who signed it on the cli on instantation
    pub expiration: Expiration,
}

#[cw_serde]
pub enum ExecuteMsg {
    ///Account can send funds to contract and specify two beneficiary accounts
    SendCoins {amount: Option<Vec<Coin>>, beneficiary1: Addr, beneficiary2: Addr},
    /// Beneficiary account can withdraw funds from its balance
    WithdrawCoins {account: Addr, quantity: Option<Vec<Coin>>}, 
    //fees collected by SendCoins goes to contract owner
    //if account balance is zero, remove the account, test for this
    //if contract expires send each beneficiary account value back to the sending account owner
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
   
    #[returns(ConfigResponse)]
      GetStateResponse {},


    //GetOwner returns the current owner as a json-encoded string
    #[returns(ConfigResponse)]
       GetOwner {},

    #[returns(ConfigResponse)]
    GetBalance { account : Addr,},
}

// We define a custom struct for each query response

pub type ConfigResponse = State;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OwnerResponse {
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BalanceResponse {
    pub balance: Option<Vec<Coin>>, 
}

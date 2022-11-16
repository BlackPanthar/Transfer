use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;
use cosmwasm_std::Coin;
use cw_utils::Expiration;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {

    pub owner: Addr, 
    pub source: String,
    pub sent_coins: i32,
    pub beneficiary_1: String,
    pub beneficiary_2: String,
    pub beneficiary1_balance: i32,
    pub beneficiary2_balance: i32,
    pub expiration: Expiration,
}


pub const STATE: Item<State> = Item::new("state");

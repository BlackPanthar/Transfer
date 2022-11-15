use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;
use cosmwasm_std::Coin;
use cw_utils::Expiration;
use cw_storage_plus::Item;
//use cosmwasm_std::Storage;
//use cosmwasm_storage::ReadonlySingleton;
//use cosmwasm_storage::Singleton;
//use cosmwasm_storage::singleton;
//use cosmwasm_storage::singleton_read;


// configuration instance key. config object will be saved under this key.
//pub static CONFIG_KEY: &[u8] = b"config";
//pub static CONFIG_KEY: &[u8] = b"state";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {

    pub owner: Addr, 
    pub source: Option<Addr>,
    pub sent_coins: Option<Vec<Coin>>,
    pub beneficiary_1: Option<Addr>,
    pub beneficiary_2: Option<Addr>,
    pub beneficiary1_balance: Option<Vec<Coin>>,
    pub beneficiary2_balance: Option<Vec<Coin>>,
    pub expiration: Expiration,
}
/* 
pub fn config(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, CONFIG_KEY)
  }
  
  pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, CONFIG_KEY)
  }
*/
pub const STATE: Item<State> = Item::new("state");

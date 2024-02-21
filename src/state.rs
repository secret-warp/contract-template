use schemars::JsonSchema;
use secret_toolkit::storage::Item;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, StdResult, Storage};

pub static CONFIG: Item<Config> = Item::new(b"config");

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub message: String,
}

impl Config {
    pub fn read_config(storage: &dyn Storage) -> StdResult<Config> {
        CONFIG.load(storage)
    }

    pub fn save_config(&self, storage: &mut dyn Storage) -> StdResult<()> {
        CONFIG.save(storage, &self)
    }
}

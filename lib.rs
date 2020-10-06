// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::wee_alloc;
use near_sdk::{env, ext_contract, near_bindgen};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Clone, Default, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct Field {
    pub field_id: u64,
    pub pool_id: String,
    pub name: String,
    pub value: String,
}

type FieldsByPools = HashMap<u64, Field>;

#[ext_contract(staking_pool)]
pub trait StakingPool {
    fn get_owner_id(&self) -> String;
}

#[ext_contract(ext_self_owner)]
pub trait ExtPoolDetails {
    fn on_get_owner_id(
        &mut self,
        #[callback] get_owner_id: String,
        pool_id: String,
        name: String,
        value: String,
    ) -> bool;
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct PoolDetails {
    fields: FieldsByPools
}

#[near_bindgen]
impl PoolDetails {
    pub fn update_field(&mut self, pool_id: String, name: String, value: String) -> bool {
        let _owner_account_id_to_compare: String = env::signer_account_id().clone();

        env::log(format!("_owner_account_id_to_compare {}", _owner_account_id_to_compare).as_bytes());

        staking_pool::get_owner_id(
            &pool_id, 0, 175000000000000)
            .then(ext_self_owner::on_get_owner_id(
                _owner_account_id_to_compare,
                pool_id,
                name,
                &value,
                0,
                175000000000000,
            ));

        true
    }

    pub fn get_all_fields(&self) -> &HashMap<u64, Field> {
        &self.fields
    }

    pub fn on_get_owner_id(
        &mut self,
        #[callback] owner_id: String,
        staking_pool_account_id: String,
        pool_id: String,
        name: String,
        value: String,
    ) -> bool {
        env::log(format!("owner_id {}", owner_id).as_bytes());

        assert!(
            owner_id == staking_pool_account_id,
            "You are not the owner of pool"
        );

        assert!(
            name == "",
            "Name is empty"
        );

        let field_id = *self.fields.keys().max().unwrap_or(&0u64) + 1;

        self.fields.insert(
            field_id,
            Field {
                field_id,
                pool_id,
                name,
                value,
            },
        );

        true
    }
}
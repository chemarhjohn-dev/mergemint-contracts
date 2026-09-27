use soroban_sdk::{
    contract, contractimpl, token::TokenClient, Address, BytesN, Env, String, Symbol, Vec,
};

use crate::errors;
use crate::errors::{fail, ContractError};
use crate::events;
use crate::storage;
use crate::types::{Bounty, BountyId, BountyMeta, Contributor, Milestone};

#[contract]
pub struct MergeMintContract;

include!("mutations.rs");
include!("queries.rs");

#[contractimpl]
impl MergeMintContract {
    /// Returns a page of bounties associated with `tag`.
    ///
    /// Invalid tags fail with `InvalidTag`, matching `create_bounty` validation.
    pub fn get_bounties_by_tag(
        env: Env,
        tag: Symbol,
        offset: u32,
        limit: u32,
    ) -> Result<Vec<Bounty>, ContractError> {
        if !storage::is_valid_tag(&env, &tag) {
            fail(&env, ContractError::InvalidTag);
        }
        Ok(storage::get_bounties_by_tag(&env, &tag, offset, limit))
    }

    /// Returns the total number of bounties associated with `tag`.
    ///
    /// Invalid tags fail with `InvalidTag`, matching `create_bounty` validation.
    pub fn get_tag_count(env: Env, tag: Symbol) -> Result<u32, ContractError> {
        if !storage::is_valid_tag(&env, &tag) {
            fail(&env, ContractError::InvalidTag);
        }
        Ok(storage::get_tag_count(&env, &tag))
    }
}

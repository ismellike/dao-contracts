use std::marker::PhantomData;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_hooks::Hooks;
use cw_storage_plus::{Item, Map};

use dao_voting::{deposit::CheckedDepositInfo, pre_propose::PreProposeSubmissionPolicy};

#[cw_serde]
pub struct Config {
    /// Information about the deposit required to create a
    /// proposal. If `None`, no deposit is required.
    pub deposit_info: Option<CheckedDepositInfo>,
    /// The policy dictating who is allowed to submit proposals.
    pub submission_policy: PreProposeSubmissionPolicy,
}

pub struct PreProposeContract<InstantiateExt, ExecuteExt, QueryExt, MigrateExt, ProposalMessage> {
    /// The proposal module that this module is associated with.
    pub proposal_module: Item<'static, Addr>,
    /// The DAO (dao-dao-core module) that this module is associated
    /// with.
    pub dao: Item<'static, Addr>,
    /// The configuration for this module.
    pub config: Item<'static, Config>,
    /// Map between proposal IDs and (deposit, proposer) pairs.
    pub deposits: Map<'static, u64, (Option<CheckedDepositInfo>, Addr)>,
    /// Consumers of proposal submitted hooks.
    pub proposal_submitted_hooks: Hooks<'static>,

    // These types are used in associated functions, but not
    // assocaited data. To stop the compiler complaining about unused
    // generics, we build this phantom data.
    instantiate_type: PhantomData<InstantiateExt>,
    execute_type: PhantomData<ExecuteExt>,
    query_type: PhantomData<QueryExt>,
    migrate_type: PhantomData<MigrateExt>,
    proposal_type: PhantomData<ProposalMessage>,
}

impl<InstantiateExt, ExecuteExt, QueryExt, MigrateExt, ProposalMessage>
    PreProposeContract<InstantiateExt, ExecuteExt, QueryExt, MigrateExt, ProposalMessage>
{
    const fn new(
        proposal_key: &'static str,
        dao_key: &'static str,
        config_key: &'static str,
        deposits_key: &'static str,
        proposal_submitted_hooks_key: &'static str,
    ) -> Self {
        Self {
            proposal_module: Item::new(proposal_key),
            dao: Item::new(dao_key),
            config: Item::new(config_key),
            deposits: Map::new(deposits_key),
            proposal_submitted_hooks: Hooks::new(proposal_submitted_hooks_key),
            execute_type: PhantomData,
            instantiate_type: PhantomData,
            query_type: PhantomData,
            migrate_type: PhantomData,
            proposal_type: PhantomData,
        }
    }
}

impl<InstantiateExt, ExecuteExt, QueryExt, MigrateExt, ProposalMessage> Default
    for PreProposeContract<InstantiateExt, ExecuteExt, QueryExt, MigrateExt, ProposalMessage>
{
    fn default() -> Self {
        // Call into constant function here. Presumably, the compiler
        // is clever enough to inline this. This gives us
        // "more-or-less" constant evaluation for our default method.
        Self::new(
            "proposal_module",
            "dao",
            "config",
            "deposits",
            "proposal_submitted_hooks",
        )
    }
}

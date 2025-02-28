use cosmwasm_schema::cw_serde;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use dao_pre_propose_base::{
    error::PreProposeError,
    msg::{
        ExecuteMsg as ExecuteBase, InstantiateMsg as InstantiateBase, MigrateMsg as MigrateBase,
        QueryMsg as QueryBase,
    },
    state::PreProposeContract,
};
use dao_voting::{
    multiple_choice::{MultipleChoiceAutoVote, MultipleChoiceOptions},
    proposal::MultipleChoiceProposeMsg as ProposeMsg,
};

pub(crate) const CONTRACT_NAME: &str = "crates.io:dao-pre-propose-multiple";
pub(crate) const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cw_serde]
pub enum ProposeMessage {
    Propose {
        title: String,
        description: String,
        choices: MultipleChoiceOptions,
        vote: Option<MultipleChoiceAutoVote>,
    },
}

pub type InstantiateMsg = InstantiateBase<Empty>;
pub type ExecuteMsg = ExecuteBase<ProposeMessage, Empty>;
pub type QueryMsg = QueryBase<Empty>;
pub type MigrateMsg = MigrateBase<Empty>;

/// Internal version of the propose message that includes the
/// `proposer` field. The module will fill this in based on the sender
/// of the external message.
#[cw_serde]
enum ProposeMessageInternal {
    Propose(ProposeMsg),
}

type PrePropose = PreProposeContract<Empty, Empty, Empty, Empty, ProposeMessageInternal>;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, PreProposeError> {
    let resp = PrePropose::default().instantiate(deps.branch(), env, info, msg)?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(resp)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, PreProposeError> {
    // We don't want to expose the `proposer` field on the propose
    // message externally as that is to be set by this module. Here,
    // we transform an external message which omits that field into an
    // internal message which sets it.
    type ExecuteInternal = ExecuteBase<ProposeMessageInternal, Empty>;
    let internalized = match msg {
        ExecuteMsg::Propose {
            msg:
                ProposeMessage::Propose {
                    title,
                    description,
                    choices,
                    vote,
                },
        } => ExecuteInternal::Propose {
            msg: ProposeMessageInternal::Propose(ProposeMsg {
                proposer: Some(info.sender.to_string()),
                title,
                description,
                choices,
                vote,
            }),
        },
        ExecuteMsg::Extension { msg } => ExecuteInternal::Extension { msg },
        ExecuteMsg::Withdraw { denom } => ExecuteInternal::Withdraw { denom },
        ExecuteMsg::UpdateConfig {
            deposit_info,
            submission_policy,
        } => ExecuteInternal::UpdateConfig {
            deposit_info,
            submission_policy,
        },
        ExecuteMsg::UpdateSubmissionPolicy {
            denylist_add,
            denylist_remove,
            set_dao_members,
            allowlist_add,
            allowlist_remove,
        } => ExecuteInternal::UpdateSubmissionPolicy {
            denylist_add,
            denylist_remove,
            set_dao_members,
            allowlist_add,
            allowlist_remove,
        },
        ExecuteMsg::AddProposalSubmittedHook { address } => {
            ExecuteInternal::AddProposalSubmittedHook { address }
        }
        ExecuteMsg::RemoveProposalSubmittedHook { address } => {
            ExecuteInternal::RemoveProposalSubmittedHook { address }
        }
        ExecuteBase::ProposalCompletedHook {
            proposal_id,
            new_status,
        } => ExecuteInternal::ProposalCompletedHook {
            proposal_id,
            new_status,
        },
    };

    PrePropose::default().execute(deps, env, info, internalized)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    PrePropose::default().query(deps, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(mut deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, PreProposeError> {
    let res = PrePropose::default().migrate(deps.branch(), msg);
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    res
}

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use cw2::set_contract_version;

use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::OWNER,
};

const CONTRACT_NAME: &str = "crates.io:example";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    OWNER.save(deps.storage, &info.sender)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ChangeOwner { owner } => execute_change_owner(deps, env, info, owner),
    }
}

pub fn execute_change_owner(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    owner: String,
) -> Result<Response, ContractError> {
    let old_owner = OWNER.load(deps.storage)?;

    if old_owner != info.sender {
        return Err(ContractError::Unauthorized(
            "Only owner can call this message".to_string(),
        ));
    }

    OWNER.save(deps.storage, &deps.api.addr_validate(&owner)?)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner {} => to_json_binary(&OWNER.load(deps.storage)?),
    }
}

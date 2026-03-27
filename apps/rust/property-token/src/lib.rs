use crate::security::prevent_replay;

pub fn execute_set_metadata(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    metadata: PropertyMetadata,
    auth: Auth,
) -> Result<Response, StdError> {
    prevent_replay(&mut deps, &env, &info, auth.nonce, auth.expires_at)?;

    validate_metadata(&metadata)?;

    METADATA.save(deps.storage, &token_id, &metadata)?;

    Ok(Response::new().add_attribute("action", "set_metadata"))
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, StdError> {
    match msg {
        ExecuteMsg::SetMetadata { token_id, metadata } => {
            execute_set_metadata(deps, env, info, token_id, metadata)
        }
        ExecuteMsg::UpdateMetadata { token_id, metadata } => {
            execute_update_metadata(deps, env, info, token_id, metadata)
        }
        ExecuteMsg::Batch { msgs } => execute_batch(deps, env, info, msgs),
    }
}
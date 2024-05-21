use crate::{
    vc::{
        create_credential_hash, create_credential_jwt, get_alias_tuple, save_credential_hash,
        validate_credential_spec, validate_score,
    },
    PRINCIPAL_SCORE,
};
use ic_cdk::{api::time, caller, update};
use serde_bytes::ByteBuf;
use vc_util::issuer_api::{IssueCredentialError, PrepareCredentialRequest, PreparedCredentialData};

/// Prepares a credential based on the given request.
///
/// This function handles the preparation of a credential by validating the input specifications
/// and ensuring the principal meets the required conditions. It encapsulates the entire workflow
/// from validation to JWT creation, providing a single entry point for this operation.
///
/// # Arguments
///
/// * `req` - A `PrepareCredentialRequest` containing the signed ID alias and credential specification.
///
/// # Returns
///
/// * `Ok(PreparedCredentialData)` - Contains the prepared context as a ByteBuf if successful.
/// * `Err(IssueCredentialError)` - Contains an error if validation or credential creation fails.
#[update(name = "prepare_credential")]
async fn vc_prepare_credential(
    req: PrepareCredentialRequest,
) -> Result<PreparedCredentialData, IssueCredentialError> {
    let alias_tuple = get_alias_tuple(&req.signed_id_alias, &caller(), time().into())?;
    validate_credential_spec(&req.credential_spec)?;

    let principal: [u8; 29] = alias_tuple.id_dapp.as_slice()[..29].try_into().unwrap();
    let score = PRINCIPAL_SCORE.with_borrow(|s| match s.get(&principal) {
        Some(score) => Ok(score),
        None => Err(IssueCredentialError::UnauthorizedSubject(
            "No score registered for principal".to_string(),
        )),
    })?;

    validate_score(score, &req.credential_spec)?;

    let credential_jwt = create_credential_jwt(&req.credential_spec, &alias_tuple)?;

    let credential_hash = create_credential_hash(&alias_tuple, &credential_jwt)?;

    save_credential_hash(&alias_tuple, credential_hash)?;

    Ok(PreparedCredentialData {
        prepared_context: Some(ByteBuf::from(credential_jwt.as_bytes())),
    })
}

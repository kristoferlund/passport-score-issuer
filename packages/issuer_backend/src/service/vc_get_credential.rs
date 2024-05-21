use crate::vc::{
    create_credential_hash, create_jws, get_alias_tuple, get_signature, validate_credential_spec,
};
use ic_cdk::{api::time, caller, query};
use vc_util::issuer_api::{GetCredentialRequest, IssueCredentialError, IssuedCredentialData};

/// Handles the retrieval of a credential, performing necessary validations and transformations.
///
/// This function validates the provided credential specification and the alias tuple,
/// extracts the prepared context, computes the credential hash, retrieves the signature,
/// and creates a JWS (JSON Web Signature) from the credential JWT.
///
/// # Arguments
///
/// * `req` - A `GetCredentialRequest` containing the signed ID alias, credential specification, and prepared context.
///
/// # Returns
///
/// * `Ok(IssuedCredentialData)` - Contains the JWS of the credential if successful.
/// * `Err(IssueCredentialError)` - If validation or any step in the process fails.
#[query(name = "get_credential")]
fn vc_get_credential(
    req: GetCredentialRequest,
) -> Result<IssuedCredentialData, IssueCredentialError> {
    let alias_tuple = get_alias_tuple(&req.signed_id_alias, &caller(), time().into())?;

    validate_credential_spec(&req.credential_spec)?;

    let prepared_context = req
        .prepared_context
        .ok_or_else(|| IssueCredentialError::Internal("Missing prepared_context".to_string()))?;

    let credential_jwt = String::from_utf8(prepared_context.into_vec())
        .map_err(|_| IssueCredentialError::Internal("Invalid prepared_context".to_string()))?;

    let credential_hash = create_credential_hash(&alias_tuple, &credential_jwt)?;

    let sig = get_signature(&alias_tuple, credential_hash)?;

    let vc_jws = create_jws(&alias_tuple, &credential_jwt, sig.as_slice())?;

    Ok(IssuedCredentialData { vc_jws })
}

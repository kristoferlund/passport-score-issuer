use candid::Nat;
use ic_cdk::update;
use vc_util::issuer_api::{Icrc21ConsentInfo, Icrc21Error, Icrc21VcConsentMessageRequest};

use crate::vc::{get_credential_min_score, validate_credential_spec};

/// Handles the generation of a consent message for credential sharing.
///
/// This function validates the credential specification and the user's language preference.
/// It then retrieves the minimum score required for the credential and generates a consent message.
///
/// # Arguments
///
/// * `req` - An `Icrc21VcConsentMessageRequest` containing the credential specification and user preferences.
///
/// # Returns
///
/// * `Ok(Icrc21ConsentInfo)` - Contains the consent message and language if successful.
/// * `Err(Icrc21Error)` - Contains an error if validation or message creation fails.
#[update]
async fn vc_consent_message(
    req: Icrc21VcConsentMessageRequest,
) -> Result<Icrc21ConsentInfo, Icrc21Error> {
    // Validate the credential specification.
    validate_credential_spec(&req.credential_spec).map_err(|_| Icrc21Error::GenericError {
        error_code: Nat::from(400u32),
        description: "Unsupported or invalid credential type".to_string(),
    })?;

    // Ensure the language preference is supported.
    if req.preferences.language != "en-US" {
        return Err(Icrc21Error::GenericError {
            error_code: Nat::from(400u32),
            description: "Unsupported language".to_string(),
        });
    }

    // Retrieve the minimum score required for the credential.
    let min_score =
        get_credential_min_score(&req.credential_spec).map_err(|_| Icrc21Error::GenericError {
            error_code: Nat::from(400u32),
            description: "minScore not found in credential type".to_string(),
        })?;

    // Construct the consent message.
    let consent_message = format!("<h1>Gitcoin Passport Score</h1><br/>Minimum Score: {min_score}<br/><br/>Sharing the credential DOES NOT mean revealing your exact Passport Score, Ethereum address or other personal information.");

    Ok(Icrc21ConsentInfo {
        consent_message,
        language: "en".to_string(),
    })
}

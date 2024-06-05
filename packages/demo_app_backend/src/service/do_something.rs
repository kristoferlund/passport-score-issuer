use std::collections::HashMap;

use ic_cdk::query;
use vc_util::{
    issuer_api::{ArgumentValue, CredentialSpec},
    validate_ii_presentation_and_claims, VcFlowSigners,
};

use crate::SETTINGS;

/// Validate a Verifiable Presentation (VP) JWT against the credential specification and signers
/// defined in the settings.
///
/// The VP JWT is assumed to contain two credentials:
///
/// 1. An `InternetIdentityIdAlias` credential issued by the II canister, linking current user
/// principal to an id alias.
///
/// 2. A `GitcoinPassportScore` credential issued by the issuer canister, proving the user's
/// Gitcoin Passport Score.
///
/// The validation incorporates multiple factors such as credential type and argument values.
///
/// # Arguments
///
/// * `vp_jwt` - A JWT string representing the VP to be validated.
#[query]
fn do_something(vp_jwt: String) -> String {
    SETTINGS.with_borrow(|settings_opt| {
        let settings = settings_opt
            .as_ref()
            .expect("Settings should be initialized");

        // Unique identifier of the caller principal
        let effective_vc_subject = ic_cdk::api::caller();

        // Current system time in nanoseconds, used for validation timestamp
        let current_time_ns: u128 = ic_cdk::api::time() as u128;

        // Define the two signers involved in the Verifiable Credential flow
        let vc_flow_signers = VcFlowSigners {
            ii_canister_id: settings.ii_canister_id,
            ii_origin: "https://identity.ic0.app/".to_string(),
            issuer_canister_id: settings.issuer_canister_id,
            issuer_origin: "https://ycons-daaaa-aaaal-qja3q-cai.icp0.io/".to_string(),
        };

        // Define the expected credential specification for the VP. This spec should match the
        // credential type and argument values in the VP.
        let vc_spec = CredentialSpec {
            credential_type: "GitcoinPassportScore".to_string(),
            arguments: Some(HashMap::from([(
                "minScore".to_string(),
                ArgumentValue::Int(1),
            )])),
        };

        // Validate the VP JWT against the specified signers and credential specifications
        match validate_ii_presentation_and_claims(
            &vp_jwt,
            effective_vc_subject,
            &vc_flow_signers,
            &vc_spec,
            &settings.ic_root_key_raw,
            current_time_ns,
        ) {
            Ok(_) => "✅ Success, the credential is valid.".to_string(),
            Err(e) => format!("🛑 Error: {:?}", e),
        }
    })
}

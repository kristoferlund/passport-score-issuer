use candid::{candid_method, Nat};
use ic_cdk::update;
use vc_util::issuer_api::{Icrc21ConsentInfo, Icrc21Error, Icrc21VcConsentMessageRequest};

use crate::CREDENTIAL_TYPE;

#[update]
#[candid_method]
async fn vc_consent_message(
    req: Icrc21VcConsentMessageRequest,
) -> Result<Icrc21ConsentInfo, Icrc21Error> {
    ic_cdk::println!("vc_consent_message called");

    if req.credential_spec.credential_type != CREDENTIAL_TYPE {
        return Err(Icrc21Error::GenericError {
            error_code: Nat::from(400u32),
            description: "Unsupported credential type".to_string(),
        });
    }

    if (req.preferences.language.to_lowercase() != "en")
        && (req.preferences.language.to_lowercase() != "de")
    {
        return Err(Icrc21Error::GenericError {
            error_code: Nat::from(400u32),
            description: "Unsupported language".to_string(),
        });
    }

    Ok(Icrc21ConsentInfo {
        consent_message: "consent message".to_string(),
        language: "en".to_string(),
    })
}

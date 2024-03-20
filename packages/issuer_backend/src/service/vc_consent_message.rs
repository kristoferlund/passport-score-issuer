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
    ic_cdk::println!("{:?}", req);

    if req.credential_spec.credential_type != CREDENTIAL_TYPE {
        return Err(Icrc21Error::GenericError {
            error_code: Nat::from(400u32),
            description: "Unsupported credential type".to_string(),
        });
    }

    if req.preferences.language != "en-US" {
        return Err(Icrc21Error::GenericError {
            error_code: Nat::from(400u32),
            description: "Unsupported language".to_string(),
        });
    }

    Ok(Icrc21ConsentInfo {
        consent_message: "<h1>Gitcoin Passport Score</h1>\nDo you want to share your Gitcoin Passport Score?\n\nSharing this credential DOES NOT mean revealing your Etherum address or other personal information.".to_string(),
        language: "en".to_string(),
    })
}

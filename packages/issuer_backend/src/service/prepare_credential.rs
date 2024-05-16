use crate::{
    certified_data::update_root_hash,
    vc::{authorize_vc_request, build_credential_jwt, calculate_seed, CredentialParams},
    CREDENTIAL_TYPE, PRINCIPAL_SCORE, SIGNATURES,
};
use candid::candid_method;
use canister_sig_util::CanisterSigPublicKey;
use ic_cdk::{api::time, caller, update};
use serde_bytes::ByteBuf;
use serde_json::json;
use vc_util::{
    did_for_principal,
    issuer_api::{
        CredentialSpec, IssueCredentialError, PrepareCredentialRequest, PreparedCredentialData,
    },
    vc_signing_input, vc_signing_input_hash, AliasTuple,
};

const VC_EXPIRATION_PERIOD_NS: u64 = 15 * 60 * 1_000_000_000; // 15 minutes

fn prepare_credential_jwt(
    credential_spec: &CredentialSpec,
    alias_tuple: &AliasTuple,
) -> Result<String, IssueCredentialError> {
    // Check if the credential spec is valid
    if credential_spec.credential_type != CREDENTIAL_TYPE {
        return Err(IssueCredentialError::UnsupportedCredentialSpec(
            "Unsupported credential type".to_string(),
        ));
    }
    let principal: [u8; 29] = alias_tuple.id_dapp.as_slice()[..29]
        .try_into()
        .map_err(|_| IssueCredentialError::InvalidIdAlias("Invalid id alias".to_string()))?;

    let c = PRINCIPAL_SCORE.with_borrow(|s| match s.get(&principal) {
        Some(score) => Ok(score),
        None => Err("Principal not found".to_string()),
    });
    let d = c.unwrap();

    let params = CredentialParams {
        spec: credential_spec.clone(),
        score: d,
        subject_id: did_for_principal(alias_tuple.id_alias),
        credential_id_url: "https://example.edu/credentials/3732".to_string(),
        issuer_url: "https://example.edu".to_string(),
        expiration_timestamp_s: exp_timestamp_s(),
    };

    Ok(build_credential_jwt(params))
}

fn exp_timestamp_s() -> u32 {
    ((time() + VC_EXPIRATION_PERIOD_NS) / 1_000_000_000) as u32
}

// fn validate_claim(arg: &str, ii_issuer_url: &str, iss: Option<&str>) -> _ {
//     todo!()
// }

#[update]
#[candid_method]
async fn prepare_credential(
    req: PrepareCredentialRequest,
) -> Result<PreparedCredentialData, IssueCredentialError> {
    ic_cdk::println!("prepare_credential called");
    ic_cdk::println!(
        "prepare_credential, caller principal: {:?}",
        caller().to_string()
    );

    let alias_tuple = match authorize_vc_request(&req.signed_id_alias, &caller(), time().into()) {
        Ok(alias_tuple) => alias_tuple,
        Err(err) => return Err(err),
    };

    ic_cdk::println!(
        "prepare_credential, id_alias principal: {:?}",
        alias_tuple.id_alias.to_string()
    );
    ic_cdk::println!(
        "prepare_credential, id_dapp principal: {:?}",
        alias_tuple.id_dapp.to_string()
    );

    let credential_jwt = match prepare_credential_jwt(&req.credential_spec, &alias_tuple) {
        Ok(credential) => credential,
        Err(err) => return Result::<PreparedCredentialData, IssueCredentialError>::Err(err),
    };

    let seed = calculate_seed(&alias_tuple.id_alias);
    let canister_id = ic_cdk::id();
    let canister_sig_pk = CanisterSigPublicKey::new(canister_id, seed.to_vec());

    let signing_input =
        vc_signing_input(&credential_jwt, &canister_sig_pk).expect("failed getting signing_input");
    let msg_hash = vc_signing_input_hash(&signing_input);

    SIGNATURES.with(|sigs| {
        let mut sigs = sigs.borrow_mut();
        sigs.add_signature(seed.as_ref(), msg_hash);
    });
    update_root_hash();
    ic_cdk::println!("prepare_credential returning");
    ic_cdk::println!("{:?}", credential_jwt);
    Ok(PreparedCredentialData {
        prepared_context: Some(ByteBuf::from(credential_jwt.as_bytes())),
    })
}

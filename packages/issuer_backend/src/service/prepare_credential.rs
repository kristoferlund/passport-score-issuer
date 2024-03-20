use crate::{
    certified_data::update_root_hash, CREDENTIAL_TYPE, PRINCIPAL_SCORE, SETTINGS, SIGNATURES,
};
use candid::{candid_method, Principal};
use canister_sig_util::{hash_bytes, CanisterSigPublicKey};
use ic_cdk::{api::time, caller, update};
use ic_certification::Hash;
use serde_bytes::ByteBuf;
use vc_util::{
    build_credential_jwt, did_for_principal, get_verified_id_alias_from_jws,
    issuer_api::{
        CredentialSpec, IssueCredentialError, PrepareCredentialRequest, PreparedCredentialData,
    },
    vc_signing_input, vc_signing_input_hash, AliasTuple, CredentialParams,
};

const VC_EXPIRATION_PERIOD_NS: u64 = 15 * 60 * 1_000_000_000; // 15 minutes

fn prepare_credential_jwt(
    credential_spec: &CredentialSpec,
    alias_tuple: &AliasTuple,
) -> Result<String, String> {
    // Check if the credential spec is valid
    if credential_spec.credential_type != CREDENTIAL_TYPE {
        return Err("Invalid credential type".to_string());
    }
    let principal: [u8; 29] = alias_tuple.id_dapp.as_slice()[..29]
        .try_into()
        .map_err(|_| "Invalid principal".to_string())?;

    let c = PRINCIPAL_SCORE.with_borrow(|s| match s.get(&principal) {
        Some(score) => Ok(score),
        None => Err("Principal not found".to_string()),
    });
    let d = c.unwrap();

    let params = CredentialParams {
        spec: credential_spec.clone(),
        subject_id: did_for_principal(alias_tuple.id_alias),
        credential_id_url: "https://example.edu/credentials/3732".to_string(),
        issuer_url: "https://example.edu".to_string(),
        expiration_timestamp_s: exp_timestamp_s(),
    };

    Ok(build_credential_jwt(params))

    // unimplemented!()
}

fn exp_timestamp_s() -> u32 {
    ((time() + VC_EXPIRATION_PERIOD_NS) / 1_000_000_000) as u32
}

fn calculate_seed(principal: &Principal) -> Hash {
    // IMPORTANT: In a real dapp the salt should be set to a random value.
    let dummy_salt = [5u8; 32];

    let mut bytes: Vec<u8> = vec![];
    bytes.push(dummy_salt.len() as u8);
    bytes.extend_from_slice(&dummy_salt);

    let principal_bytes = principal.as_slice();
    bytes.push(principal_bytes.len() as u8);
    bytes.extend(principal_bytes);
    hash_bytes(bytes)
}

#[update]
#[candid_method]
async fn prepare_credential(
    req: PrepareCredentialRequest,
) -> Result<PreparedCredentialData, IssueCredentialError> {
    ic_cdk::println!("prepare_credential called");
    // ic_cdk::println!("{:?}", req);

    SETTINGS.with_borrow(|settings_opt| {
        let settings = settings_opt.as_ref().unwrap();

        ic_cdk::println!("CANISTER ID {:?}", settings.ii_canister_id);
        ic_cdk::println!("ROOT KEY {:?}", settings.ic_root_key_raw);
        let alias_result = get_verified_id_alias_from_jws(
            &req.signed_id_alias.credential_jws,
            &caller(),
            &settings.ii_canister_id,
            &settings.ic_root_key_raw,
            time().into(),
        );
        let alias_tuple = alias_result.unwrap();

        ic_cdk::println!("{:?}", alias_tuple);
        // Ok(AliasTuple { id_alias: Principal { len: 29, bytes: [137, 169, 70, 136, 73, 17, 22, 165, 23, 216, 170, 177, 98, 3, 233, 40, 199, 67, 184, 52, 229, 173, 118, 236, 105, 165, 216, 28, 2] }, id_dapp: Principal { len: 29, bytes: [137, 95, 168, 65, 107, 47, 113, 125, 11, 94, 5, 61, 170, 72, 251, 198, 147, 48, 145, 97, 185, 41, 117, 14, 131, 88, 84, 220, 2] } })

        let credential_jwt = prepare_credential_jwt(&req.credential_spec, &alias_tuple).unwrap();

        let seed = calculate_seed(&alias_tuple.id_alias);
        let canister_id = ic_cdk::id();
        let canister_sig_pk = CanisterSigPublicKey::new(canister_id, seed.to_vec());
        let signing_input = vc_signing_input(&credential_jwt, &canister_sig_pk)
            .expect("failed getting signing_input");
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
        // unimplemented!()
    })

    // unimplemented!()
}

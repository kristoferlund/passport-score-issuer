use canister_sig_util::CanisterSigPublicKey;
use ic_cdk::{api::time, caller, query};
use vc_util::{
    issuer_api::{
        ArgumentValue, CredentialSpec, GetCredentialRequest, IssueCredentialError,
        IssuedCredentialData,
    },
    vc_jwt_to_jws, vc_signing_input, vc_signing_input_hash,
};

use crate::{
    vc::{authorize_vc_request, calculate_seed},
    ASSETS, CREDENTIAL_TYPE, SIGNATURES,
};

// Verifies that the credential spec `spec` contains an argument `expected_argument`
// with the value `expected_value`.
fn verify_single_argument(
    spec: &CredentialSpec,
    expected_argument: &str,
    expected_value: ArgumentValue,
) -> Result<(), String> {
    fn missing_argument_error(
        spec: &CredentialSpec,
        expected_argument: &str,
    ) -> Result<(), String> {
        Err(format!(
            "Missing argument '{}' for credential {}",
            expected_argument, spec.credential_type
        ))
    }

    let Some(arguments) = &spec.arguments else {
        return missing_argument_error(spec, expected_argument);
    };
    let Some(value) = arguments.get(expected_argument) else {
        return missing_argument_error(spec, expected_argument);
    };

    if value != &expected_value {
        return Err(format!(
            "Unsupported value for argument '{}': expected '{}', got '{}'",
            expected_argument, expected_value, value
        ));
    }

    let unexpected_arguments: Vec<&String> = arguments
        .keys()
        .filter(|k| k.as_str() != expected_argument)
        .collect();
    if !unexpected_arguments.is_empty() {
        return Err(format!(
            "Unexpected arguments for credential {}: {:?}",
            spec.credential_type, unexpected_arguments
        ));
    }
    Ok(())
}

// fn prepare_credential_jwt(
//     credential_spec: &CredentialSpec,
//     alias_tuple: &AliasTuple,
// ) -> Result<String, IssueCredentialError> {
//     // Check if the credential spec is valid
//     if credential_spec.credential_type != CREDENTIAL_TYPE {
//         return Err(IssueCredentialError::UnsupportedCredentialSpec(
//             "Unsupported credential type".to_string(),
//         ));
//     }
//     let principal: [u8; 29] = alias_tuple.id_dapp.as_slice()[..29]
//         .try_into()
//         .map_err(|_| IssueCredentialError::InvalidIdAlias("Invalid id alias".to_string()))?;

//     let c = PRINCIPAL_SCORE.with_borrow(|s| match s.get(&principal) {
//         Some(score) => Ok(score),
//         None => Err("Principal not found".to_string()),
//     });
//     let d = c.unwrap();

//     let params = CredentialParams {
//         spec: credential_spec.clone(),
//         subject_id: did_for_principal(alias_tuple.id_alias),
//         credential_id_url: "https://example.edu/credentials/3732".to_string(),
//         issuer_url: "https://example.edu".to_string(),
//         expiration_timestamp_s: exp_timestamp_s(),
//     };

//     Ok(build_credential_jwt(params))
// }

fn internal_error(msg: &str) -> IssueCredentialError {
    IssueCredentialError::Internal(String::from(msg))
}
#[query]
fn get_credential(req: GetCredentialRequest) -> Result<IssuedCredentialData, IssueCredentialError> {
    ic_cdk::println!("get_credential called");
    ic_cdk::println!("{:?}", req);

    // if let Err(err) = authorize_vc_request(&req.signed_id_alias, &caller(), time().into()) {
    //     return Result::<IssuedCredentialData, IssueCredentialError>::Err(err);
    // };

    let alias_tuple = match authorize_vc_request(&req.signed_id_alias, &caller(), time().into()) {
        Ok(alias_tuple) => alias_tuple,
        Err(err) => return Result::<IssuedCredentialData, IssueCredentialError>::Err(err),
    };

    if &req.credential_spec.credential_type != CREDENTIAL_TYPE {
        return Result::<IssuedCredentialData, IssueCredentialError>::Err(
            IssueCredentialError::UnsupportedCredentialSpec(
                "Unsupported credential type".to_string(),
            ),
        );
    }

    let prepared_context = match req.prepared_context {
        Some(context) => context,
        None => {
            return Result::<IssuedCredentialData, IssueCredentialError>::Err(internal_error(
                "missing prepared_context",
            ))
        }
    };
    let credential_jwt = match String::from_utf8(prepared_context.into_vec()) {
        Ok(s) => s,
        Err(_) => {
            return Result::<IssuedCredentialData, IssueCredentialError>::Err(internal_error(
                "invalid prepared_context",
            ))
        }
    };

    let seed = calculate_seed(&alias_tuple.id_alias);
    let canister_id = ic_cdk::id();
    let canister_sig_pk = CanisterSigPublicKey::new(canister_id, seed.to_vec());

    let signing_input =
        vc_signing_input(&credential_jwt, &canister_sig_pk).expect("failed getting signing_input");
    let message_hash = vc_signing_input_hash(&signing_input);
    let sig_result = SIGNATURES.with(|sigs| {
        let sig_map = sigs.borrow();
        let certified_assets_root_hash = ASSETS.with_borrow(|assets| assets.root_hash());
        sig_map.get_signature_as_cbor(&seed, message_hash, Some(certified_assets_root_hash))
    });
    let sig = match sig_result {
        Ok(sig) => sig,
        Err(e) => {
            return Result::<IssuedCredentialData, IssueCredentialError>::Err(
                IssueCredentialError::SignatureNotFound(format!(
                    "signature not prepared or expired: {}",
                    e
                )),
            );
        }
    };
    let vc_jws =
        vc_jwt_to_jws(&credential_jwt, &canister_sig_pk, &sig).expect("failed constructing JWS");

    ic_cdk::println!("vc_jws: {:?}", vc_jws);
    Result::<IssuedCredentialData, IssueCredentialError>::Ok(IssuedCredentialData { vc_jws })

    // unimplemented!()
}

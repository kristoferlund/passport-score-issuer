use candid::candid_method;
use ic_cdk::update;
use vc_util::issuer_api::{IssueCredentialError, PrepareCredentialRequest, PreparedCredentialData};

#[update]
#[candid_method]
async fn prepare_credential(
    _req: PrepareCredentialRequest,
) -> Result<PreparedCredentialData, IssueCredentialError> {
    unimplemented!()
}

// #[update]
// #[candid_method]
// async fn prepare_credential(
//     req: PrepareCredentialRequest,
// ) -> Result<PreparedCredentialData, IssueCredentialError> {
//     let alias_tuple = match authorize_vc_request(&req.signed_id_alias, &caller(), time().into()) {
//         Ok(alias_tuple) => alias_tuple,
//         Err(err) => return Err(err),
//     };

//     let credential_jwt = match prepare_credential_jwt(&req.credential_spec, &alias_tuple) {
//         Ok(credential) => credential,
//         Err(err) => return Result::<PreparedCredentialData, IssueCredentialError>::Err(err),
//     };
//     let seed = calculate_seed(&alias_tuple.id_alias);
//     let canister_id = ic_cdk::id();
//     let canister_sig_pk = CanisterSigPublicKey::new(canister_id, seed.to_vec());
//     let signing_input =
//         vc_signing_input(&credential_jwt, &canister_sig_pk).expect("failed getting signing_input");
//     let msg_hash = vc_signing_input_hash(&signing_input);

//     SIGNATURES.with(|sigs| {
//         let mut sigs = sigs.borrow_mut();
//         sigs.add_signature(seed.as_ref(), msg_hash);
//     });
//     update_root_hash();
//     Ok(PreparedCredentialData {
//         prepared_context: Some(ByteBuf::from(credential_jwt.as_bytes())),
//     })
// }

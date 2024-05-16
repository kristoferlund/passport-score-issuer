use candid::Principal;
use canister_sig_util::hash_bytes;
use ic_certification::Hash;
use identity_core::common::{Timestamp, Url};
use identity_core::convert::FromJson;
use identity_credential::credential::{Credential, CredentialBuilder, Subject};
use serde_json::json;
use vc_util::{
    get_verified_id_alias_from_jws,
    issuer_api::{CredentialSpec, IssueCredentialError, SignedIdAlias},
    AliasTuple,
};

use crate::SETTINGS;

pub struct CredentialParams {
    pub spec: CredentialSpec,
    pub score: f32,
    pub subject_id: String,
    pub credential_id_url: String,
    pub issuer_url: String,
    pub expiration_timestamp_s: u32,
}

pub fn authorize_vc_request(
    alias: &SignedIdAlias,
    expected_vc_subject: &Principal,
    current_time_ns: u128,
) -> Result<AliasTuple, IssueCredentialError> {
    SETTINGS.with_borrow(|settings_opt| {
        let settings = settings_opt.as_ref().unwrap();

        if let Ok(alias_tuple) = get_verified_id_alias_from_jws(
            &alias.credential_jws,
            expected_vc_subject,
            &settings.ii_canister_id,
            &settings.ic_root_key_raw,
            current_time_ns,
        ) {
            return Ok(alias_tuple);
        }
        Err(IssueCredentialError::InvalidIdAlias(
            "id alias could not be verified".to_string(),
        ))
    })
}

pub fn calculate_seed(principal: &Principal) -> Hash {
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

pub fn build_credential_jwt(params: CredentialParams) -> String {
    let mut subject_json = json!({"id": params.subject_id});
    subject_json
        .as_object_mut()
        .unwrap()
        .insert(params.spec.credential_type.clone(), json!(params.score));
    let subject = Subject::from_json_value(subject_json).unwrap();
    let expiration_date = Timestamp::from_unix(params.expiration_timestamp_s as i64)
        .expect("internal: failed computing expiration timestamp");
    let credential: Credential = CredentialBuilder::default()
        .id(Url::parse(params.credential_id_url).unwrap())
        .issuer(Url::parse(params.issuer_url).unwrap())
        .type_(params.spec.credential_type)
        .subject(subject)
        .expiration_date(expiration_date)
        .build()
        .unwrap();
    credential.serialize_jwt().unwrap()
}

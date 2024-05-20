use crate::{certified_data::update_root_hash, ASSETS, CREDENTIAL_TYPE, SETTINGS, SIGNATURES};
use candid::Principal;
use canister_sig_util::{hash_bytes, CanisterSigPublicKey};
use ic_cdk::api::time;
use ic_certification::Hash;
use identity_core::{
    common::{Timestamp, Url},
    convert::FromJson,
};
use identity_credential::credential::{Credential, CredentialBuilder, Subject};
use serde_json::json;
use vc_util::{
    did_for_principal, get_verified_id_alias_from_jws,
    issuer_api::{ArgumentValue, CredentialSpec, IssueCredentialError, SignedIdAlias},
    vc_jwt_to_jws, vc_signing_input, vc_signing_input_hash, AliasTuple,
};

const VC_EXPIRATION_PERIOD_NS: u64 = 15 * 60 * 1_000_000_000; // 15 minutes

/// Validates that the provided credential specification matches the expected type and contains a valid argument.
///
/// The function ensures that the credential type is supported and the minimum score argument is present and positive.
pub fn validate_credential_spec(
    credential_spec: &CredentialSpec,
) -> Result<(), IssueCredentialError> {
    // Ensure the credential type is as expected
    if credential_spec.credential_type != CREDENTIAL_TYPE {
        return Err(IssueCredentialError::UnsupportedCredentialSpec(
            "Unsupported credential type".to_string(),
        ));
    }

    // Validate the presence and value of the 'minScore' argument
    if let Some(arguments) = &credential_spec.arguments {
        if arguments.len() == 1 {
            if let Some(ArgumentValue::Int(min_score)) = arguments.get("minScore") {
                if *min_score > 0 {
                    return Ok(());
                }
            }
        }
    }

    Err(IssueCredentialError::UnsupportedCredentialSpec(
        "Unsupported credential type".to_string(),
    ))
}

/// Retrieves the minimum score from the credential specification.
///
/// This function ensures that the 'minScore' argument is present and valid.
pub fn get_credential_min_score(
    credential_spec: &CredentialSpec,
) -> Result<i32, IssueCredentialError> {
    if let Some(arguments) = &credential_spec.arguments {
        if let Some(ArgumentValue::Int(min_score)) = arguments.get("minScore") {
            return Ok(*min_score);
        }
    }
    Err(IssueCredentialError::UnsupportedCredentialSpec(
        "Unsupported credential type".to_string(),
    ))
}

/// Verifies the ID alias from the signed JWT and returns the alias tuple.
///
/// This function checks the validity of the provided JWS and ensures it matches the expected subject.
pub fn get_alias_tuple(
    alias: &SignedIdAlias,
    expected_vc_subject: &Principal,
    current_time_ns: u128,
) -> Result<AliasTuple, IssueCredentialError> {
    SETTINGS.with_borrow(|settings_opt| {
        let settings = settings_opt
            .as_ref()
            .expect("Settings should be initialized");

        get_verified_id_alias_from_jws(
            &alias.credential_jws,
            expected_vc_subject,
            &settings.ii_canister_id,
            &settings.ic_root_key_raw,
            current_time_ns,
        )
        .map_err(|_| {
            IssueCredentialError::UnauthorizedSubject("JWS verification failed".to_string())
        })
    })
}

/// Generates a deterministic seed for a given principal.
///
/// Note: In a real application, the salt should be randomized to ensure uniqueness.
pub fn generate_seed(principal: &Principal) -> Hash {
    let dummy_salt = [5u8; 32]; // Static salt for example purposes

    let mut bytes: Vec<u8> = Vec::with_capacity(32 + 1 + principal.as_slice().len() + 1);
    bytes.push(dummy_salt.len() as u8);
    bytes.extend_from_slice(&dummy_salt);

    let principal_bytes = principal.as_slice();
    bytes.push(principal_bytes.len() as u8);
    bytes.extend_from_slice(principal_bytes);

    hash_bytes(bytes)
}

/// Saves the credential hash for a given alias tuple and credential JWT.
///
/// This function generates a seed, creates a signing input and its hash, and updates the root hash.
pub fn save_credential_hash(
    alias_tuple: &AliasTuple,
    credential_hash: Hash,
) -> Result<(), IssueCredentialError> {
    let seed = generate_seed(&alias_tuple.id_alias);

    SIGNATURES.with(|sigs| {
        let mut sigs = sigs.borrow_mut();
        sigs.add_signature(seed.as_ref(), credential_hash);
    });

    update_root_hash();

    Ok(())
}

/// Creates a hash for the given credential JWT.
pub fn create_credential_hash(
    alias_tuple: &AliasTuple,
    credential_jwt: &str,
) -> Result<Hash, IssueCredentialError> {
    let canister_id = ic_cdk::id();
    let seed = generate_seed(&alias_tuple.id_alias);
    let canister_sig_pk = CanisterSigPublicKey::new(canister_id, seed.to_vec());

    // Compute the signing input and handle any errors that occur.
    let signing_input = vc_signing_input(credential_jwt, &canister_sig_pk).map_err(|e| {
        IssueCredentialError::Internal(format!("Failed computing signing input: {}", e))
    })?;

    // Generate the credential hash.
    let credential_hash = vc_signing_input_hash(&signing_input);

    Ok(credential_hash)
}

/// Retrieves the signature for a given credential hash.
pub fn get_signature(
    alias_tuple: &AliasTuple,
    credential_hash: Hash,
) -> Result<Vec<u8>, IssueCredentialError> {
    let seed = generate_seed(&alias_tuple.id_alias);

    SIGNATURES
        .with(|sigs| {
            let sig_map = sigs.borrow();
            let certified_assets_root_hash = ASSETS.with_borrow(|assets| assets.root_hash());

            // Retrieve the signature from the signature map.
            sig_map.get_signature_as_cbor(&seed, credential_hash, Some(certified_assets_root_hash))
        })
        .map_err(|e| IssueCredentialError::Internal(format!("Signature not found: {}", e)))
}

/// Creates a JWS (JSON Web Signature) for the given credential JWT.
pub fn create_jws(
    alias_tuple: &AliasTuple,
    credential_jwt: &str,
    sig: &[u8],
) -> Result<String, IssueCredentialError> {
    let canister_id = ic_cdk::id();
    let seed = generate_seed(&alias_tuple.id_alias);

    let canister_sig_pk = CanisterSigPublicKey::new(canister_id, seed.to_vec());

    // Convert the JWT to a JWS and handle any errors that occur.
    vc_jwt_to_jws(credential_jwt, &canister_sig_pk, sig)
        .map_err(|e| IssueCredentialError::Internal(format!("Failed creating JWS: {}", e)))
}

/// Validates the score against the minimum score required by the credential specification.
///
/// This ensures that the principal's score meets the minimum threshold defined in the credential specification,
/// encapsulating the comparison logic and making the high-level code cleaner.
pub fn validate_score(
    score: f32,
    credential_spec: &CredentialSpec,
) -> Result<(), IssueCredentialError> {
    let min_score = get_credential_min_score(credential_spec)?;

    if score < min_score as f32 {
        return Err(IssueCredentialError::UnauthorizedSubject(
            "Insufficient score for credential".to_string(),
        ));
    }

    Ok(())
}

/// Creates a JWT for the credential.
///
/// Encapsulates the entire process of creating a JWT for a credential, from building the subject JSON
/// to setting the expiration timestamp, ensuring all details are handled in a single, focused function.
pub fn create_credential_jwt(
    credential_spec: &CredentialSpec,
    alias_tuple: &AliasTuple,
) -> Result<String, IssueCredentialError> {
    let mut subject_json = json!({"id": did_for_principal(alias_tuple.id_alias)});
    let min_score = get_credential_min_score(credential_spec)?;
    subject_json.as_object_mut().unwrap().insert(
        credential_spec.credential_type.clone(),
        json!({"minScore": min_score}),
    );
    let subject = Subject::from_json_value(subject_json).unwrap();

    let expiration_seconds = ((time() + VC_EXPIRATION_PERIOD_NS) / 1_000_000_000) as i64;
    let expiration_timestamp = Timestamp::from_unix(expiration_seconds).map_err(|e| {
        IssueCredentialError::Internal(format!("Failed computing expiration timestamp: {}", e))
    })?;

    let credential: Credential = CredentialBuilder::default()
        .id(Url::parse("https://example.edu/credentials/3732".to_string()).unwrap())
        .issuer(Url::parse("https://example.edu".to_string()).unwrap())
        .type_(credential_spec.credential_type.clone())
        .subject(subject)
        .expiration_date(expiration_timestamp)
        .build()
        .unwrap();

    Ok(credential.serialize_jwt().unwrap())
}

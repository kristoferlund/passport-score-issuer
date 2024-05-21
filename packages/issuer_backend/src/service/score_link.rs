use crate::authenticated_guard::authenticated;
use crate::eth::{create_signing_message, recover_eth_address, EthAddress, EthSignature};
use crate::passport_score_api::get_passport_score;
use crate::{ETH_PRINCIPAL, PRINCIPAL_SCORE};
use ic_cdk::{caller, update};

/// Links an Ethereum address to a principal and a passport score.
///
/// The function performs several validations:
/// 1. Ensures the principal and address are not already registered.
/// 2. Validates the Ethereum address and signature.
/// 3. Verifies the recovered address from the signature matches the provided address.
/// 4. Fetches the passport score for the address.
/// 5. Registers or updates the address and its corresponding principal.
///
/// # Arguments
///
/// * `signature` - The Ethereum signature provided for authentication.
/// * `address` - The Ethereum address to be registered or refreshed.
///
/// # Returns
///
/// * `Ok(f32)` - The passport score if registration or refresh is successful.
/// * `Err(String)` - An error message if any validation or operation fails.
#[update(guard = authenticated)]
pub async fn score_link(signature: String, address: String) -> Result<f32, String> {
    let caller_principal: [u8; 29] = caller().as_slice()[..29]
        .try_into()
        .map_err(|_| "Invalid principal".to_string())?;

    // Validate and create an EthAddress from the provided string.
    let address = EthAddress::new(&address)?;

    // Validate and create an EthSignature from the provided string.
    let signature = EthSignature::new(&signature)?;

    // Generate the signing message to verify the signature.
    let message = create_signing_message(&address, &caller());

    // Recover the address from the signature and ensure it matches the provided address.
    let recovered_address = recover_eth_address(&message, &signature)?;
    if recovered_address != address.as_str() {
        return Err("Invalid signature".to_string());
    }

    // Fetch the passport score for the Ethereum address.
    let score = get_passport_score(&address).await?;

    // Register or update the principal and address.
    let mut eth_already_registered = false;
    let mut principal_already_registered = false;

    ETH_PRINCIPAL.with_borrow(|map| {
        if let Some(principal) = map.get(&address.as_hash()) {
            if caller_principal != principal {
                return Err("Invalid eth/principal combination".to_string());
            }
            eth_already_registered = true;
        }
        Ok::<(), String>(())
    })?;

    PRINCIPAL_SCORE.with_borrow(|s| {
        if s.contains_key(&caller_principal) {
            principal_already_registered = true;
        }
        Ok::<(), String>(())
    })?;

    if eth_already_registered && principal_already_registered {
        // Update existing score
        PRINCIPAL_SCORE.with_borrow_mut(|s| {
            s.insert(caller_principal, score);
        });
    } else if !eth_already_registered && !principal_already_registered {
        // Register new principal and address
        ETH_PRINCIPAL.with_borrow_mut(|e| {
            e.insert(address.as_hash(), caller_principal);
        });

        PRINCIPAL_SCORE.with_borrow_mut(|s| {
            s.insert(caller_principal, score);
        });
    } else {
        // Either the address or principal is already registered, but not both
        return Err("Principal or address already registered".to_string());
    }

    Ok(score)
}

use crate::eth::create_signing_message;
use crate::guard::authenticated;
use crate::passport::get_passport_score;
use crate::{
    eth::{recover_eth_address, EthAddress, EthSignature},
    ETH_PRINCIPAL, PRINCIPAL_SCORE,
};
use ic_cdk::update;

#[update(guard = authenticated)]
pub async fn create_credential(signature: String, address: String) -> Result<f32, String> {
    let caller = ic_cdk::caller();
    let principal: [u8; 29] = caller.as_slice()[..29]
        .try_into()
        .map_err(|_| "Invalid principal".to_string())?;

    ic_cdk::println!("create_credential called");
    ic_cdk::println!("principal: {:?}", principal);

    // Function can only be called once per principal.
    PRINCIPAL_SCORE.with_borrow(|s| {
        if s.contains_key(&principal) {
            return Err("Principal already registered".to_string());
        }
        Ok(())
    })?;

    // Create an EthAddress from the string. This validates the address.
    let address = EthAddress::new(&address)?;

    // Function can only be called once per address.
    ETH_PRINCIPAL.with_borrow(|e| {
        if e.contains_key(&address.as_hash()) {
            return Err("Address already registered".to_string());
        }
        Ok(())
    })?;

    // Create an EthSignature from the string. This validates the signature.
    let signature = EthSignature::new(&signature)?;

    // Create a message string to recover the address from the signature.
    let message = create_signing_message(&address, &caller);

    // Compare the address recovered from the signature with the address provided.
    let recovered_address = recover_eth_address(&message, &signature)?;
    if recovered_address != address.as_str() {
        return Err("Invalid signature".to_string());
    }

    let score = get_passport_score(&address).await?;

    ETH_PRINCIPAL.with_borrow_mut(|e| {
        e.insert(address.as_hash(), principal);
    });

    PRINCIPAL_SCORE.with_borrow_mut(|s| {
        s.insert(principal, score);
    });

    Ok(score)
}

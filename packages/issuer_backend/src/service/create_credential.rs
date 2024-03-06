use crate::guard::authenticated;
use candid::Principal;
use ic_cdk::update;

use crate::{
    eth::{recover_eth_address, EthAddress, EthSignature},
    ETH_PRINCIPAL, PRINCIPAL_SCORE,
};

pub fn register_message(address: &EthAddress, principal: &Principal) -> String {
    let msg = format!(
      "Sign this message to link your Ethereum address to your Internet Computer identity.\n\nEthereum address: {}\n\nInternet Computer principal: {}",
      address.as_str(),
      principal.to_text()
  );

    msg
}

#[update(guard = authenticated)]
pub fn create_credential(signature: String, address: String) -> Result<f32, String> {
    let caller = ic_cdk::caller();
    let principal: [u8; 29] = caller.as_slice()[..29]
        .try_into()
        .map_err(|_| "Invalid principal".to_string())?;

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
    let message = register_message(&address, &caller);

    // Compare the address recovered from the signature with the address provided.
    let recovered_address = recover_eth_address(&message, &signature)?;
    if recovered_address != address.as_str() {
        return Err("Invalid signature".to_string());
    }

    ETH_PRINCIPAL.with_borrow_mut(|e| {
        e.insert(address.as_hash(), principal);
    });

    PRINCIPAL_SCORE.with_borrow_mut(|s| {
        s.insert(principal, 15.0); // Dummy score for now
    });

    Ok(15.0)
}

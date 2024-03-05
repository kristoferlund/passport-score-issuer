use candid::Principal;
use ic_cdk::update;

use crate::eth::{recover_eth_address, EthAddress, EthSignature};

pub fn register_message(address: &EthAddress, principal: &Principal) -> String {
    let msg = format!(
      "Sign this message to link your\nEthereum address to your\nInternet Computer identity.\n\nEthereum address: {}\n\nInternet Computer principal: {}",
      address.as_str(),
      principal.to_text()
  );

    msg
}

#[update]
pub fn register_address(signature: String, address: String) -> Result<(), String> {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        return Err("Not authorized".to_string());
    }

    // Create an EthAddress from the string. This validates the address.
    let address = EthAddress::new(&address)?;

    // Create an EthSignature from the string. This validates the signature.
    let signature = EthSignature::new(&signature)?;

    // Create a message string to recover the address from the signature.
    let message = register_message(&address, &caller);

    let recovered_address = recover_eth_address(&message, &signature)?;
    if recovered_address != address.as_str() {
        return Err("Invalid signature".to_string());
    }

    Ok(())
}

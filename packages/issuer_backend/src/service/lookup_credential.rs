use crate::{eth::EthAddress, guard::authenticated, ETH_PRINCIPAL};
use ic_cdk::query;

use crate::PRINCIPAL_SCORE;

#[query(guard = authenticated)]
pub fn lookup_credential(address: String) -> Result<f32, String> {
    let caller = ic_cdk::caller();
    let caller: [u8; 29] = caller.as_slice()[..29]
        .try_into()
        .map_err(|_| "Invalid principal".to_string())?;

    // Create an EthAddress from the string. This validates the address.
    let address = EthAddress::new(&address)?;

    ETH_PRINCIPAL.with_borrow(|e| match e.get(&address.as_hash()) {
        Some(p) => {
            if p.eq_ignore_ascii_case(&caller) {
                return Ok(());
            }
            Err("Address not linked to principal".to_string())
        }
        None => Err("Address not found".to_string()),
    })?;

    PRINCIPAL_SCORE.with_borrow(|s| match s.get(&caller) {
        Some(score) => Ok(score),
        None => Err("Principal not found".to_string()),
    })
}

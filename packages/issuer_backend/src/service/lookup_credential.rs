use crate::authenticated_guard::authenticated;
use ic_cdk::{caller, query};

use crate::PRINCIPAL_SCORE;

#[query(guard = authenticated)]
pub fn lookup_credential() -> Result<f32, String> {
    let caller_principal: [u8; 29] = caller().as_slice()[..29]
        .try_into()
        .map_err(|_| "Invalid principal".to_string())?;

    PRINCIPAL_SCORE.with_borrow(|s| match s.get(&caller_principal) {
        Some(score) => Ok(score),
        None => Err("Principal not found".to_string()),
    })
}

use candid::{CandidType, Principal};
use serde::Deserialize;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct SettingsInput {
    pub ic_root_key_der: Vec<u8>,
    pub ii_canister_id: Principal,
}

pub struct Settings {
    pub ic_root_key_raw: Vec<u8>,
    pub ii_canister_id: Principal,
}

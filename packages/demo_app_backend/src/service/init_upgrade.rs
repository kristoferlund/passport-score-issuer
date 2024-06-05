use crate::{
    certified_data::init_assets,
    settings::{Settings, SettingsInput},
    SETTINGS,
};
use canister_sig_util::extract_raw_root_pk_from_der;
use ic_cdk::{init, post_upgrade};

#[init]
async fn init(settings_input: SettingsInput) {
    save_settings(settings_input);
    init_assets();
}

#[post_upgrade]
fn upgrade(settings_input: SettingsInput) {
    save_settings(settings_input);
    init_assets();
}

fn save_settings(settings_input: SettingsInput) {
    SETTINGS.with_borrow_mut(|settings| {
        *settings = Some(Settings {
            ii_canister_id: settings_input.ii_canister_id,
            ic_root_key_raw: extract_raw_root_pk_from_der(&settings_input.ic_root_key_der).unwrap(),
        });
    });
}

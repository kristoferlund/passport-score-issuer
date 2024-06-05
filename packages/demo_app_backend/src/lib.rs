mod certified_data;
mod http;
mod service;
mod settings;

use asset_util::CertifiedAssets;
use canister_sig_util::signature_map::SignatureMap;
use http::{HttpRequest, HttpResponse};
use ic_cdk::export_candid;
use settings::{Settings, SettingsInput};
use std::cell::RefCell;

thread_local! {

    static SETTINGS: RefCell<Option<Settings>> = const { RefCell::new(None) };
    static SIGNATURES : RefCell<SignatureMap> = RefCell::new(SignatureMap::default());
    static ASSETS: RefCell<CertifiedAssets> = RefCell::new(CertifiedAssets::default());

}

export_candid!();

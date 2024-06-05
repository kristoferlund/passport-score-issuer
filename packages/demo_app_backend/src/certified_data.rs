use crate::{ASSETS, SIGNATURES};
use asset_util::{collect_assets, CertifiedAssets};
use canister_sig_util::signature_map::LABEL_SIG;
use ic_cdk::api::set_certified_data;
use ic_certification::{fork_hash, labeled_hash};
use include_dir::{include_dir, Dir};

static ASSET_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../demo_app_frontend/dist");

pub fn update_root_hash() {
    SIGNATURES.with_borrow(|sigs| {
        ASSETS.with_borrow(|assets| {
            let prefixed_root_hash = fork_hash(
                &assets.root_hash(),
                &labeled_hash(LABEL_SIG, &sigs.root_hash()),
            );
            set_certified_data(&prefixed_root_hash[..]);
        })
    })
}

fn fixup_html(html: &str) -> String {
    let canister_id = ic_cdk::api::id();

    // the string we are replacing here is inserted by vite during the front-end build
    html.replace(
          r#"<script type="module" crossorigin src="/index.js"></script>"#,
          &format!(r#"<script data-canister-id="{canister_id}" type="module" crossorigin src="/index.js"></script>"#).to_string()
      )
}

pub fn init_assets() {
    let assets = collect_assets(&ASSET_DIR, Some(fixup_html));
    ASSETS.with_borrow_mut(|certified_assets| {
        *certified_assets = CertifiedAssets::certify_assets(assets, &crate::http::static_headers());
    });
    update_root_hash()
}

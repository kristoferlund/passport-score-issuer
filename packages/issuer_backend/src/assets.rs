use asset_util::{collect_assets, CertifiedAssets};
use include_dir::{include_dir, Dir};

use crate::{certified_data::update_root_hash, ASSETS};

static ASSET_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../issuer_frontend/dist");

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

use crate::{
    http::{static_headers, HttpRequest, HttpResponse},
    ASSETS, SIGNATURES,
};
use canister_sig_util::signature_map::LABEL_SIG;
use ic_cdk::query;
use ic_certification::{labeled_hash, pruned};
use serde_bytes::ByteBuf;

#[query]
pub fn http_request(req: HttpRequest) -> HttpResponse {
    let parts: Vec<&str> = req.url.split('?').collect();
    let path = parts[0];
    let sigs_root_hash =
        SIGNATURES.with_borrow(|sigs| pruned(labeled_hash(LABEL_SIG, &sigs.root_hash())));
    let maybe_asset = ASSETS.with_borrow(|assets| {
        assets.get_certified_asset(path, req.certificate_version, Some(sigs_root_hash))
    });

    let mut headers = static_headers();
    match maybe_asset {
        Some(asset) => {
            headers.extend(asset.headers);
            HttpResponse {
                status_code: 200,
                body: ByteBuf::from(asset.content),
                headers,
            }
        }
        None => HttpResponse {
            status_code: 404,
            headers,
            body: ByteBuf::from(format!("Asset {} not found.", path)),
        },
    }
}

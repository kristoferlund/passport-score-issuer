use ic_cdk::update;
use vc_util::issuer_api::{DerivationOriginData, DerivationOriginError, DerivationOriginRequest};

#[update]
async fn derivation_origin(
    _req: DerivationOriginRequest,
) -> Result<DerivationOriginData, DerivationOriginError> {
    ic_cdk::println!("derivation_origin called");
    Ok(DerivationOriginData {
        origin: format!("https://{}.icp0.io", ic_cdk::api::id()),
    })
}

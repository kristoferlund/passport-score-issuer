use ic_cdk::update;
use vc_util::issuer_api::{DerivationOriginData, DerivationOriginError, DerivationOriginRequest};

#[update]
async fn derivation_origin(
    _req: DerivationOriginRequest,
) -> Result<DerivationOriginData, DerivationOriginError> {
    ic_cdk::println!("derivation_origin called");
    // origin: format!("https://{}.icp0.io", ic_cdk::api::id()),
    let origin = format!("http://{}.localhost:4943", ic_cdk::api::id());
    ic_cdk::println!("{}", origin.as_str());
    Ok(DerivationOriginData { origin })
}

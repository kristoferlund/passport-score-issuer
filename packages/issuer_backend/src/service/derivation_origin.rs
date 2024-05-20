use ic_cdk::update;
use vc_util::issuer_api::{DerivationOriginData, DerivationOriginError, DerivationOriginRequest};

#[update]
async fn derivation_origin(
    _req: DerivationOriginRequest,
) -> Result<DerivationOriginData, DerivationOriginError> {
    let dfx_network = option_env!("DFX_NETWORK").unwrap();
    let origin = match dfx_network {
        "local" => format!("http://{}.localhost:4943", ic_cdk::api::id()),
        "ic" => format!("https://{}.icp0.io", ic_cdk::api::id()),
        _ => {
            return Err(DerivationOriginError::Internal(
                "Invalid DFX_NETWORK".to_string(),
            ))
        }
    };
    Ok(DerivationOriginData { origin })
}

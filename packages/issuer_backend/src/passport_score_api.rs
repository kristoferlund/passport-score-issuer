use crate::eth::EthAddress;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod, TransformContext,
};
use serde_json::Value;

///
/// Get the Gitcoin Passport score for an Ethereum address from the Gitcoin Passport API.
///
pub async fn get_passport_score(address: &EthAddress) -> Result<f32, String> {
    // Since the Gitcoin Passport API does not accept IPv6 connections, we use a Cloudflare Worker
    // to proxy the request to the API. Source code for the Worker can be found at
    // https://github.com/kristoferlund/passport-score-api-proxy
    let url = format!(
        "https://passport-score-proxy.kristofer-977.workers.dev/submit/{address}",
        address = address.as_str()
    );

    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None,
        transform: Some(TransformContext::from_name(
            "transform".to_string(),
            serde_json::to_vec(&Vec::<u8>::new()).unwrap(),
        )),
        headers: vec![],
    };

    match http_request(request, 30_000_000_000).await {
        Ok((response,)) => {
            // Convert the response body to a string
            let body = String::from_utf8(response.body)
                .map_err(|_| "Couldn't read Gitcoin Passport API response".to_string())?;

            // Parse the response body as JSON
            let v: Value = serde_json::from_str(&body)
                .map_err(|_| "Invalid JSON in Gitcoin Passport API response".to_string())?;

            // Access the "score" field and convert it to a f32
            match v["score"].as_str() {
                Some(score) => Ok(score.parse().unwrap_or(0.0)),
                None => Err("Gitcoin Passport API response doesn't contain a score".to_string()),
            }
        }
        Err((_, m)) => Err(format!("Gitcoin Passport API request failed: {}", m)),
    }
}

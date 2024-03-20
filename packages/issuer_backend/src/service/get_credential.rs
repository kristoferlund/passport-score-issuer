use ic_cdk::query;
use vc_util::issuer_api::{GetCredentialRequest, IssueCredentialError, IssuedCredentialData};

#[query]
fn get_credential(req: GetCredentialRequest) -> Result<IssuedCredentialData, IssueCredentialError> {
    ic_cdk::println!("get_credential called");
    ic_cdk::println!("{:?}", req);
    unimplemented!()
}

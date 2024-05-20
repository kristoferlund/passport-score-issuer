use candid::Principal;

pub fn authenticated() -> Result<(), String> {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        return Err("Not authorized".to_string());
    }

    Ok(())
}

use crate::{ASSETS, SIGNATURES};
use canister_sig_util::signature_map::LABEL_SIG;
use ic_cdk::api::set_certified_data;
use ic_certification::{fork_hash, labeled_hash};

pub fn update_root_hash() {
    SIGNATURES.with_borrow(|sigs| {
        ASSETS.with_borrow(|assets| {
            ic_cdk::println!("Asssets: {:?}", assets);

            let prefixed_root_hash = fork_hash(
                // NB: Labels added in lexicographic order.
                &assets.root_hash(),
                &labeled_hash(LABEL_SIG, &sigs.root_hash()),
            );

            set_certified_data(&prefixed_root_hash[..]);
        })
    })
}

//! The Cipher key manager.
use std::collections::HashSet;

use oasis_core_keymanager::policy::TrustedSigners;
use oasis_core_runtime::consensus::verifier::TrustRoot;

/// Determine whether the build is for Testnet.
///
/// If the crate version has a pre-release component (e.g. 2.4.0-testnet) then the build is
/// classified as Testnet. If there is no such component (e.g. 2.4.0) then it is classified as
/// Mainnet.
const fn is_testnet() -> bool {
    !env!("CARGO_PKG_VERSION_PRE").is_empty()
}

/// Trusted key manager policy signer set.
pub fn trusted_policy_signers() -> TrustedSigners {
    if is_testnet() {
        // Testnet.
        TrustedSigners {
            signers: HashSet::from([
                "b27b3d0245d4cbd78be8e04e473f36abee350fcfbc438000313db1bb06117a43".into(),
                "c37cbd0345965fda84fbaa372a01fc840b7b66eebfeb66dfdd35bb3e801f2cf3".into(),
                "df8ca9fc78ce2c01f8217e8ce7aa582e8952545f412426fe07d42ca119e12166".into(),
                "c3a091299c0b91b75df719045765e6aabd3a4b7164208d7bcde3cfc4af30eead".into(),
            ]),
            threshold: 3,
        }
    } else {
        // Mainnet. TODO
        panic!("the pontusx keymanager is not yet deployable on mainnet");
    }
}

/// Trust root.
pub fn trust_root() -> TrustRoot {
    if is_testnet() {
        // Testnet.
        TrustRoot {
            height: 21012543,
            hash: "d370f92680c69c4de27cf86f9b669ae59407eb69068ab879672da9c93e2c2bf3".into(),
            runtime_id: "400000000000000000000000000000000000000000000000c0fb685b92338f68".into(),
            chain_context: "0b91b8e4e44b2003a7c5e23ddadb5e14ef5345c0ebcb3ddcae07fa2f244cab76"
                .to_string(),
        }
    } else {
        // Mainnet. TODO
        /*TrustRoot {
            height: 19127589,
            hash: "300714be28f462d7ac6442b7ae05fa8421f2c53e5a48a135dd793fa183443bc1".into(),
            runtime_id: "400000000000000000000000000000000000000000000000bda9cfedb46428e5".into(),
            chain_context: "bb3d748def55bdfb797a2ac53ee6ee141e54cd2ab2dc2375f4a0703a178e6e55"
                .to_string(),
        }*/
        panic!("the pontusx keymanager is not yet deployable on mainnet");
    }
}

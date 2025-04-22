//! The key manager ParaTime.
use oasis_core_keymanager::runtime::init::new_keymanager;
use oasis_core_runtime::{common::version::Version, config::Config, version_from_cargo};

use pontusx_keymanager::{trust_root, trusted_policy_signers};

/// Entrypoint.
pub fn main() {
    // Create a new Oasis Core key manager instance using the specified trusted signers.
    let init = new_keymanager(trusted_policy_signers());

    // Start the runtime.
    oasis_core_runtime::start_runtime(
        init,
        Config {
            version: version_from_cargo!(),
            trust_root: Some(trust_root()),
            ..Default::default()
        },
    );
}

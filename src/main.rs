mod art;
mod version_check;

use std::{env, sync::LazyLock};

use miasma::{Miasma, MiasmaConfig, MiasmaError};

static CONFIG: LazyLock<MiasmaConfig> = LazyLock::new(MiasmaConfig::new);

fn main() -> Result<(), MiasmaError> {
    // Print the banner if the user is viewing help menu
    if env::args().any(|arg| arg == "-h" || arg == "--help") {
        art::print_miasma_ascii_art();
    }
    // Otherwise trigger parsing then print (don't print on failures or version check)
    _ = CONFIG.port;
    art::print_miasma_ascii_art();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("miasma-thread")
        .build()
        .unwrap()
        .block_on(async {
            tokio::spawn(version_check::check_for_new_version());
            let shutdown_signal = async {
                tokio::signal::ctrl_c()
                    .await
                    .expect("Failed to register shutdown listener");
            };

            let miasma = Miasma::new(&CONFIG).await?;

            CONFIG.print_config_info();

            miasma.run(shutdown_signal).await
        })
}

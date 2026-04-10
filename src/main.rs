use std::sync::LazyLock;

use miasma::{Miasma, MiasmaConfig, check_for_new_version};

static CONFIG: LazyLock<MiasmaConfig> = LazyLock::new(MiasmaConfig::new);

fn main() -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("miasma-thread")
        .build()
        .unwrap()
        .block_on(async {
            tokio::spawn(check_for_new_version());
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

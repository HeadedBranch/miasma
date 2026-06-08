mod cli;

use cli::{App, AppArgs};
use miasma::MiasmaError;
use std::env;

fn main() -> Result<(), MiasmaError> {
    // Print the banner if the user is viewing help menu
    if env::args().any(|arg| arg == "-h" || arg == "--help") {
        cli::print_miasma_ascii_art();
    }
    // Otherwise trigger parsing then print (don't print on failures or version check)
    let args = AppArgs::parse_args();
    cli::print_miasma_ascii_art();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("miasma-thread")
        .build()
        .unwrap()
        .block_on(async {
            tokio::spawn(cli::check_for_new_version());
            let shutdown_signal = async {
                tokio::signal::ctrl_c()
                    .await
                    .expect("Failed to register shutdown listener");
            };

            let app = App::new(args.clone()).await?;

            args.print_config_info();

            app.run(shutdown_signal).await
        })
}

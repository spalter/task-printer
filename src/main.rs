use clap::Parser;
use escpos::errors::Result as EscposResult;

mod api;
mod cli;
mod printer;

use api::start_api_server;
use cli::{Args, run_cli_print};

/// Main entry point of the application.
///
/// Parses command line arguments and either starts an API server or runs a CLI print job.
/// When the `--api` flag is provided, starts an HTTP server on the specified port.
/// Otherwise, executes a single print job with the provided arguments.
///
/// # Returns
///
/// * `EscposResult<()>` - Ok(()) on success, or an ESC/POS error on failure
fn main() -> EscposResult<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    let args = Args::parse();

    if args.api {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            start_api_server(args.api_port).await.unwrap();
        });
        Ok(())
    } else {
        run_cli_print(args)
    }
}

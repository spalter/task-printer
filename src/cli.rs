use clap::Parser;
use std::io::{self, Read};
use escpos::errors::Result as EscposResult;
use crate::printer::{PrintTask, print_task};

#[derive(Parser)]
#[command(name = "taskprinter")]
#[command(about = "A CLI tool to print tasks to an ESC/POS printer")]
pub struct Args {
    #[arg(short, long)]
    pub title: Option<String>,

    #[arg(short, long)]
    pub message: Option<String>,

    #[arg(short, long)]
    pub date: Option<String>,

    #[arg(short, long)]
    pub encode: Option<bool>,

    #[arg(short, long)]
    pub address: Option<String>,

    #[arg(short, long)]
    pub port: Option<u16>,

    #[arg(short, long)]
    pub codepage: Option<String>,

    #[arg(long)]
    pub api: bool,

    #[arg(long, default_value = "3000")]
    pub api_port: u16,
}

/// Runs a CLI print job with the provided arguments.
/// 
/// This function processes command line arguments to create and execute a print task.
/// If no message is provided via arguments, it reads from stdin. The title defaults
/// to "INCOMMING MESSAGE" if not specified.
/// 
/// # Arguments
/// 
/// * `args` - The parsed command line arguments containing print job configuration
/// 
/// # Returns
/// 
/// * `EscposResult<()>` - Ok(()) on successful print, or an ESC/POS error on failure
/// 
/// # Examples
/// 
/// ```
/// use taskprinter::cli::{Args, run_cli_print};
/// 
/// let args = Args {
///     title: Some("Test".to_string()),
///     message: Some("Hello World".to_string()),
///     // ... other fields
/// };
/// run_cli_print(args).expect("Print failed");
/// ```
pub fn run_cli_print(args: Args) -> EscposResult<()> {
    let title = args.title.unwrap_or_else(|| "INCOMMING MESSAGE".to_string());
    let message = match args.message {
        Some(msg) => msg,
        None => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .map_err(|e| {
                    eprintln!("Error reading from stdin: {}", e);
                    std::process::exit(1);
                })
                .unwrap();
            buffer.trim().to_string()
        }
    };
    
    print_task(PrintTask {
        title: Some(title),
        message,
        date: args.date,
        encode: args.encode,
        address: args.address,
        port: args.port,
        codepage: args.codepage,
    })
}

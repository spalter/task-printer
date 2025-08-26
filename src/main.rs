use chrono::Local;
use clap::Parser;
use escpos::printer::Printer;
use escpos::printer_options::PrinterOptions;
use escpos::utils::*;
use escpos::{driver::*, errors::Result};
use std::io::{self, Read};

#[derive(Parser)]
#[command(name = "taskprinter")]
#[command(about = "A CLI tool to print tasks to an ESC/POS printer")]
struct Args {
    #[arg(short, long)]
    title: Option<String>,

    #[arg(short, long)]
    message: Option<String>,

    #[arg(short, long)]
    date: Option<String>,

    #[arg(short, long)]
    encode: Option<bool>,

    #[arg(short, long)]
    address: Option<String>,

    #[arg(short, long)]
    port: Option<u16>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Set default title to "TASK" if not provided
    let title = args.title.unwrap_or_else(|| "TASK".to_string());

    // Get message from args or read from stdin
    let message = match args.message {
        Some(msg) => msg,
        None => {
            // Read from stdin
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

    let date_str = args
        .date
        .unwrap_or_else(|| Local::now().format("%d/%m/%Y").to_string());

    let address = args.address.unwrap_or_else(|| "taskbob".to_string());
    let port = args.port.unwrap_or(9100);
    let driver = NetworkDriver::open(&address, port, None)?;
    let mut binding = Printer::new(driver, Protocol::default(), Some(PrinterOptions::default()));
    let printer = binding
        .debug_mode(Some(DebugMode::Dec))
        .init()?
        .smoothing(true)?
        .justify(JustifyMode::LEFT)?
        .reverse(true)?
        .bold(false)?
        .writeln(&format!("{} - {}", title, date_str))?
        .feed()?;

    if args.encode == Some(true) {
        printer
            .qrcode_option(
                &message,
                QRCodeOption::new(QRCodeModel::Model1, 6, QRCodeCorrectionLevel::M),
            )?
            .feed()?;
    } else {
        printer
            .justify(JustifyMode::LEFT)?
            .reverse(false)?
            .underline(UnderlineMode::None)?
            .size(2, 2)?
            .writeln(&message)?
            .feed()?;
    }

    printer.print_cut()?;
    Ok(())
}

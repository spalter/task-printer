use chrono::Local;
use escpos::printer::Printer;
use escpos::printer_options::PrinterOptions;
use escpos::utils::*;
use escpos::{driver::*, errors::Result as EscposResult};

/// Represents a print task with all necessary configuration options.
/// 
/// This struct contains all the information needed to print a message to an ESC/POS printer,
/// including optional formatting, network settings, and encoding preferences.
#[derive(Debug, Clone)]
pub struct PrintTask {
    /// Optional title for the print job. Defaults to "INCOMMING MESSAGE" if not provided.
    pub title: Option<String>,
    /// The main message content to be printed.
    pub message: String,
    /// Optional date string. If not provided, uses current date in DD/MM/YYYY format.
    pub date: Option<String>,
    /// Whether to encode the message as a QR code instead of plain text.
    pub encode: Option<bool>,
    /// Network address of the printer. Defaults to "taskbob" if not provided.
    pub address: Option<String>,
    /// Network port of the printer. Defaults to 9100 if not provided.
    pub port: Option<u16>,
    /// Character encoding codepage. Supports PC850, ISO8859_15, WPC1252, PC437, ISO8859_7.
    pub codepage: Option<String>,
}

/// Prints a task to an ESC/POS printer over the network.
/// 
/// This function connects to a network printer and prints the provided task.
/// It handles formatting, character encoding, and supports both text and QR code output.
/// The printer connection is established using TCP/IP.
/// 
/// # Arguments
/// 
/// * `task` - A `PrintTask` containing all the print job configuration
/// 
/// # Returns
/// 
/// * `EscposResult<()>` - Ok(()) on successful print, or an ESC/POS error on failure
/// 
/// # Behavior
/// 
/// - Sets up printer with specified codepage and formatting options
/// - Prints a header with title and date in reverse/bold text
/// - If `encode` is true, prints the message as a QR code
/// - If `encode` is false or None, prints the message as large text
/// - Performs a cut operation after printing
/// 
/// # Examples
/// 
/// ```
/// use taskprinter::printer::{PrintTask, print_task};
/// 
/// let task = PrintTask {
///     title: Some("Alert".to_string()),
///     message: "System backup completed".to_string(),
///     date: None, // Will use current date
///     encode: Some(false),
///     address: Some("192.168.1.100".to_string()),
///     port: Some(9100),
///     codepage: Some("PC850".to_string()),
/// };
/// 
/// print_task(task).expect("Failed to print");
/// ```
pub fn print_task(task: PrintTask) -> EscposResult<()> {
    let title = task.title.unwrap_or_else(|| "INCOMMING MESSAGE".to_string());
    let date_str = task
        .date
        .unwrap_or_else(|| Local::now().format("%d/%m/%Y").to_string());
    let address = task.address.unwrap_or_else(|| "taskbob".to_string());
    let port = task.port.unwrap_or(9100);
    let codepage = match task.codepage.as_deref() {
        Some("PC850") => PageCode::PC850,
        Some("ISO8859_15") => PageCode::ISO8859_15,
        Some("WPC1252") => PageCode::WPC1252,
        Some("PC437") => PageCode::PC437,
        Some("ISO8859_7") => PageCode::ISO8859_7,
        _ => PageCode::PC850,
    };
    
    let driver = NetworkDriver::open(&address, port, None)?;
    let mut binding = Printer::new(driver, Protocol::default(), Some(PrinterOptions::default()));
    let printer = binding
        .debug_mode(Some(DebugMode::Dec))
        .init()?
        .page_code(codepage)?
        .smoothing(true)?
        .justify(JustifyMode::LEFT)?
        .reverse(true)?
        .bold(false)?
        .writeln(&format!("{} - {}", title, date_str))?
        .feed()?;

    if task.encode == Some(true) {
        printer
            .qrcode_option(
                &task.message,
                QRCodeOption::new(QRCodeModel::Model1, 6, QRCodeCorrectionLevel::M),
            )?
            .feed()?;
    } else {
        printer
            .justify(JustifyMode::LEFT)?
            .reverse(false)?
            .underline(UnderlineMode::None)?
            .size(2, 2)?
            .writeln(&task.message)?
            .feed()?;
    }

    printer.print_cut()?;
    Ok(())
}

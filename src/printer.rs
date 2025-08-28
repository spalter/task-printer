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
    let address = task
        .address
        .clone()
        .unwrap_or_else(|| "taskbob".to_string());
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
    let message;

    if task.title.is_some() {
        message = generate_task_string(&task);
    } else {
        message = generate_note_string(&task);
    }

    let printer = binding
        .debug_mode(Some(DebugMode::Dec))
        .init()?
        .page_code(codepage)?
        .smoothing(true)?
        .justify(JustifyMode::LEFT)?
        .reverse(false)?
        .size(2, 2)?
        .writeln(&message)?
        .feed()?;

    printer.print_cut()?;
    Ok(())
}

/// Prints a QR code representation of the message to an ESC/POS printer over the network.
/// This function connects to a network printer and prints the provided task as a QR code.
/// It handles character encoding and establishes the printer connection using TCP/IP.
/// 
/// # Arguments
/// 
/// * `task` - A `PrintTask` containing all the print job configuration
/// 
/// # Returns
/// 
/// * `EscposResult<()>` - Ok(()) on successful print, or an ESC/POS error on failure
pub fn print_qr_code(task: PrintTask) -> EscposResult<()> {
    let address = task
        .address
        .clone()
        .unwrap_or_else(|| "taskbob".to_string());
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
        .justify(JustifyMode::CENTER)?
        .reverse(false)?
        .qrcode(&task.message)?
        .feed()?;

    printer.print_cut()?;
    Ok(())
}

/// Generates a formatted string representation of a note.
/// This function creates a visually appealing box around the message,
/// including a fixed "NOTE" title at the top. It ensures that lines do not exceed
/// a maximum width for better readability.
/// 
/// # Arguments
/// 
/// * `task` - A reference to the `PrintTask` to be formatted
/// 
/// # Returns
/// 
/// * `String` - The formatted string ready for printing
pub fn generate_note_string(task: &PrintTask) -> String {
    let max_width = 24;
    let frame_header = "┌──────────────────────┐";
    let frame_title_box = "│        NOTE          │";
    let frame_separator = "├──────────────────────┤";
    let frame_body = "│ {} │";
    let frame_footer = "└──────────────────────┘";

    let mut output = format!("{}\n", frame_header);
    output.push_str(&format!("{}\n", frame_title_box));
    output.push_str(&format!("{}\n", frame_separator));
    for line in task.message.lines() {
        let mut start = 0;
        while start < line.len() {
            let end = usize::min(start + max_width, line.len());
            let segment = &line[start..end];
            output.push_str(&format!(
                "{}\n",
                frame_body.replace("{}", &format!("{:<20}", segment)[..20])
            ));
            start += max_width;
        }
    }
    output.push_str(&format!("{}", frame_footer));

    output
}

/// Generates a formatted string representation of the print task.
/// This function creates a visually appealing box around the message,
/// including the title and date at the top. It ensures that lines do not exceed
/// a maximum width for better readability.
///
/// # Arguments
///
/// * `task` - A reference to the `PrintTask` to be formatted
///
/// # Returns
///
/// * `String` - The formatted string ready for printing
pub fn generate_task_string(task: &PrintTask) -> String {
    let max_width = 24;
    let title = task.title.clone().unwrap_or_else(|| "NOTE".to_string());
    let frame_header = "┌──────────────────────┐";
    let frame_title_box = "│ {title} {date} │";
    let frame_separator = "├──────────────────────┤";
    let frame_body = "│ {} │";
    let frame_footer = "└──────────────────────┘";
    let date_str = task
        .date
        .clone()
        .unwrap_or_else(|| Local::now().format("%d/%m/%Y").to_string());
    let short_date_day_month = &date_str[0..5];

    let mut output = format!("{}\n", frame_header);
    output.push_str(&format!(
        "{}\n",
        frame_title_box
            .replace("{title}", &format!("{:<14}", title)[..14])
            .replace("{date}", &format!("{:>5}", short_date_day_month)[..5])
    ));
    output.push_str(&format!("{}\n", frame_separator));
    for line in task.message.lines() {
        let mut start = 0;
        while start < line.len() {
            let end = usize::min(start + max_width, line.len());
            let segment = &line[start..end];
            output.push_str(&format!(
                "{}\n",
                frame_body.replace("{}", &format!("{:<20}", segment)[..20])
            ));
            start += max_width;
        }
    }
    output.push_str(&format!("{}", frame_footer));

    output
}

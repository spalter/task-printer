# TaskPrinter

A command-line tool for printing tasks to ESC/POS thermal printers over a network connection.

## Installation

### Prerequisites

- Rust toolchain (install from [rustup.rs](https://rustup.rs/))
- ESC/POS compatible thermal printer with network connectivity

### Build from Source

1. Clone or download the repository
2. Navigate to the project directory
3. Build the application:

```bash
cargo build --release
```

The executable will be available at `target/release/taskprinter.exe` (Windows) or `target/release/taskprinter` (Linux/macOS).

## Usage

### Basic Usage

```bash
# Print with title and message
taskprinter -t "URGENT TASK" -m "Complete the quarterly report by EOD"

# Use default title "TASK"
taskprinter -m "Pick up groceries"

# Pipe text input (uses default title "TASK")
echo "Meeting at 3 PM" | taskprinter

# Pipe with custom title
echo "Server maintenance required" | taskprinter -t "IT ALERT"
```

### Command Line Options

| Option | Short | Long | Description | Default |
|--------|-------|------|-------------|---------|
| Title | `-t` | `--title` | Task title | "TASK" |
| Message | `-m` | `--message` | Task message | (reads from stdin if not provided) |
| Date | `-d` | `--date` | Custom date | Current date (DD/MM/YYYY) |
| Encode | `-e` | `--encode` | Generate QR code for message | false |
| Address | `-a` | `--address` | Printer IP address | "192.168.1.93" |
| Port | `-p` | `--port` | Printer port | 9100 |

### Examples

#### Standard Task Printing

```bash
# Basic task
taskprinter -t "SHOPPING" -m "Buy milk, bread, and eggs"

# Task with custom date
taskprinter -t "APPOINTMENT" -m "Doctor visit" -d "28/08/2025"

# High priority task
taskprinter -t "URGENT" -m "Fix production server"
```

#### Using Stdin (Pipe Input)

```bash
# Pipe simple text
echo "Take out the trash" | taskprinter

# Pipe with custom title
echo "System backup completed successfully" | taskprinter -t "SYSTEM"

# Pipe from file
cat todo.txt | taskprinter -t "TODO LIST"

# Pipe command output
git log --oneline -5 | taskprinter -t "RECENT COMMITS"
```

#### QR Code Generation

```bash
# Generate QR code for the message
taskprinter -t "WIFI INFO" -m "Network: GuestWiFi, Password: guest123" -e

# QR code with URL
echo "https://example.com/task/12345" | taskprinter -t "TASK LINK" -e
```

#### Custom Printer Configuration

```bash
# Different printer address
taskprinter -a "10.0.1.100" -m "Test print"

# Custom port
taskprinter -a "printer.local" -p 9100 -m "Hello World"
```

## Output Format

The printed output follows this structure:

```txt
[TITLE] - [DATE]

[MESSAGE]
```

Example output:

```txt
URGENT TASK - 26/08/2025

Complete the quarterly report by EOD
```

When QR encoding is enabled (`-e`), the message will be printed as a QR code instead of text.

## Network Printer Setup

1. Ensure your thermal printer supports ESC/POS commands
2. Connect the printer to your network
3. Note the printer's IP address and port (typically 9100)
4. Update the default address in the command or use `-a` and `-p` options

## Development

### Dependencies

- `escpos` - ESC/POS printer control
- `clap` - Command line argument parsing
- `chrono` - Date and time handling

### Building

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with arguments
cargo run -- -t "Test" -m "Development build"
```

## License

This project is available under your preferred open source license.

## Contributing

Contributions are welcome! Please feel free to submit issues, feature requests, or pull requests.

## Changelog

### v0.1.0

- Initial release
- Basic task printing functionality
- Network printer support
- Stdin input support
- QR code generation
- Configurable printer address and port
- Automatic date formatting

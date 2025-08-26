# TaskPrinter

A command-line tool and REST API for printing tasks to ESC/POS thermal printers over a network connection.

## Installation

### Prerequisites

- Rust toolchain (install from [rustup.rs](https://rustup.rs/))
- ESC/POS compatible thermal printer with network connectivity
- For container deployment: Podman or Docker

### Build from Source

1. Clone or download the repository
2. Navigate to the project directory
3. Build the application:

```bash
cargo build --release
```

The executable will be available at `target/release/taskprinter.exe` (Windows) or `target/release/taskprinter` (Linux/macOS).

## Usage

### CLI Mode

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
| Address | `-a` | `--address` | Printer IP address | "taskbob" |
| Port | `-p` | `--port` | Printer port | 9100 |
| Codepage | `-c` | `--codepage` | Character encoding (PC850, ISO8859_15, WPC1252, PC437) | PC850 |

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

## Character Encoding Support

The application supports various character encodings to handle international characters:

| Codepage | Best For | Characters Supported |
|----------|----------|---------------------|
| **PC850** | German, Western Europe | ä, ö, ü, ß, Ä, Ö, Ü, and other Latin characters |
| **ISO8859_15** | Western Europe + Euro | All PC850 characters + € (Euro symbol) |
| **WPC1252** | Windows Western | Standard Windows-1252 character set |
| **PC437** | US/English | Basic ASCII + some extended characters |
| **ISO8859_7** | Greek | Greek alphabet characters |

**Default:** PC850 (recommended for German text)

**Usage:** Add `-c CODEPAGE` or `--codepage CODEPAGE` to your command.

## Network Printer Setup

1. Ensure your thermal printer supports ESC/POS commands
2. Connect the printer to your network
3. Note the printer's IP address and port (typically 9100)
4. Update the default address in the command or use `-a` and `-p` options

### API Mode

TaskPrinter can run as a REST API server

#### Starting the API Server

```bash
# Start API server on default port 3000
taskprinter --api

# Start API server on custom port
taskprinter --api --api-port 8080
```

#### API Endpoints

##### Health Check

```http
GET /health
GET /
```

Response:

```json
{
  "status": "healthy",
  "service": "taskprinter",
  "version": "0.1.1"
}
```

##### Print Task

```http
POST /print
Content-Type: application/json
```

Request body:

```json
{
  "title": "URGENT TASK",
  "message": "Complete the quarterly report by EOD",
  "date": "26/08/2025",
  "encode": false,
  "address": "taskbob",
  "port": 9100,
  "codepage": "PC850"
}
```

Response (success):

```json
{
  "success": true,
  "message": "Print job completed successfully"
}
```

Response (error): HTTP 500 Internal Server Error

**Required fields:** Only `message` is required. All other fields are optional and will use defaults.

#### Example API Usage

```bash
# Using curl
curl -X POST http://localhost:3000/print \
  -H "Content-Type: application/json" \
  -d '{
    "title": "TODO",
    "message": "Buy groceries"
  }'

# With custom printer settings
curl -X POST http://localhost:3000/print \
  -H "Content-Type: application/json" \
  -d '{
    "message": "Server maintenance complete",
    "address": "10.0.1.100",
    "port": 9100,
    "encode": false
  }'
```

### Container Deployment

TaskPrinter includes Podman container support for easy deployment.

#### Building the Container

```bash
podman build -t taskprinter-api .
```

#### Running the Container

```bash
podman run -d -p 3000:3000 --name taskprinter taskprinter-api
```

#### Container Environment

The container runs the API server by default on port 3000. You can access the API at:

- Health check: `http://localhost:3000/health`
- Print endpoint: `http://localhost:3000/print`

## Development

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

## Changelog

### v0.1.0

- Initial release
- Basic task printing functionality
- Network printer support
- Stdin input support
- QR code generation
- Configurable printer address and port
- Automatic date formatting

### v0.1.1

- Added addition page codes

### v0.1.2

## Credits

- [escpos-rs](https://github.com/fabienbellanger/escpos-rs)

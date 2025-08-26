use axum::{
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use crate::printer::{PrintTask, print_task};

/// Request payload for the print API endpoint.
/// 
/// This struct represents the JSON payload that clients send to the `/print` endpoint.
/// All fields except `message` are optional and will use default values if not provided.
#[derive(Deserialize)]
pub struct PrintRequest {
    /// Optional title for the print job.
    pub title: Option<String>,
    /// The message content to be printed (required).
    pub message: String,
    /// Optional date string for the print job.
    pub date: Option<String>,
    /// Whether to encode the message as a QR code.
    pub encode: Option<bool>,
    /// Network address of the target printer.
    pub address: Option<String>,
    /// Network port of the target printer.
    pub port: Option<u16>,
    /// Character encoding codepage for the printer.
    pub codepage: Option<String>,
}

/// Response payload for the print API endpoint.
/// 
/// This struct represents the JSON response sent back to clients after a print request.
#[derive(Serialize)]
pub struct PrintResponse {
    /// Whether the print operation was successful.
    pub success: bool,
    /// Human-readable message describing the result.
    pub message: String,
}

/// Starts the HTTP API server.
/// 
/// This function creates and starts an HTTP server that provides REST endpoints
/// for printing tasks. The server includes CORS support and provides both health
/// check and print functionality.
/// 
/// # Arguments
/// 
/// * `port` - The port number to bind the server to (e.g., 3000)
/// 
/// # Returns
/// 
/// * `Result<(), Box<dyn std::error::Error>>` - Ok(()) if the server shuts down gracefully,
///   or an error if the server fails to start or encounters a fatal error
/// 
/// # Endpoints
/// 
/// - `GET /` - Health check endpoint
/// - `GET /health` - Health check endpoint  
/// - `POST /print` - Print a task
/// 
/// # Examples
/// 
/// ```
/// use taskprinter::api::start_api_server;
/// 
/// #[tokio::main]
/// async fn main() {
///     start_api_server(3000).await.expect("Server failed");
/// }
/// ```
pub async fn start_api_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .route("/print", post(print_handler))
        .layer(CorsLayer::permissive());

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    println!("API server running on http://0.0.0.0:{}", port);
    
    axum::serve(listener, app).await?;
    Ok(())
}

/// Health check endpoint handler.
/// 
/// This function handles GET requests to `/` and `/health` endpoints.
/// It returns a JSON response indicating the service status and version information.
/// 
/// # Returns
/// 
/// * `Json<serde_json::Value>` - A JSON response containing service health information
/// 
/// # Response Format
/// 
/// ```json
/// {
///   "status": "healthy",
///   "service": "taskprinter", 
///   "version": "<cargo_package_version>"
/// }
/// ```
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "taskprinter",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// Print endpoint handler.
/// 
/// This function handles POST requests to `/print` endpoint. It accepts a JSON payload
/// containing print job details, converts it to a `PrintTask`, and attempts to print it.
/// 
/// # Arguments
/// 
/// * `payload` - A `PrintRequest` extracted from the JSON request body
/// 
/// # Returns
/// 
/// * `Result<Json<PrintResponse>, StatusCode>` - On success, returns a JSON response
///   with success status. On failure, returns HTTP 500 Internal Server Error.
/// 
/// # Request Format
/// 
/// ```json
/// {
///   "title": "Optional title",
///   "message": "Required message content", 
///   "date": "Optional date string",
///   "encode": false,
///   "address": "printer.local",
///   "port": 9100,
///   "codepage": "PC850"
/// }
/// ```
/// 
/// # Response Format
/// 
/// ```json
/// {
///   "success": true,
///   "message": "Print job completed successfully"
/// }
/// ```
async fn print_handler(Json(payload): Json<PrintRequest>) -> Result<Json<PrintResponse>, StatusCode> {
    let task = PrintTask {
        title: payload.title,
        message: payload.message,
        date: payload.date,
        encode: payload.encode,
        address: payload.address,
        port: payload.port,
        codepage: payload.codepage,
    };

    match print_task(task) {
        Ok(()) => Ok(Json(PrintResponse {
            success: true,
            message: "Print job completed successfully".to_string(),
        })),
        Err(e) => {
            eprintln!("Print error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}


mod handlers;
mod store;
mod ticket;
mod handlers;
mod store;

use axum::{
    routing::{get, patch, post},
    Router,
};
use handlers::{create_ticket, get_ticket, health_check, list_tickets, update_ticket, AppState};
use store::TicketStore;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn init_logging() {
    // Tạo custom formatter cho console output
    let console_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .pretty();

    // Setup environment filter (có thể control qua RUST_LOG env var)
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,ticket_api=debug,axum=debug,tower_http=debug"));

    // Initialize subscriber
    tracing_subscriber::registry()
        .with(console_layer)
        .with(env_filter)
        .init();

    info!("🚀 Logging system initialized");
}

fn print_startup_banner() {
    println!("{}", colored::Colorize::bright_blue("
    ████████╗██╗ ██████╗██╗  ██╗███████╗████████╗    █████╗ ██████╗ ██╗
    ╚══██╔══╝██║██╔════╝██║ ██╔╝██╔════╝╚══██╔══╝   ██╔══██╗██╔══██╗██║
       ██║   ██║██║     █████╔╝ █████╗     ██║      ███████║██████╔╝██║
       ██║   ██║██║     ██╔═██╗ ██╔══╝     ██║      ██╔══██║██╔═══╝ ██║
       ██║   ██║╚██████╗██║  ██╗███████╗   ██║      ██║  ██║██║     ██║
       ╚═╝   ╚═╝ ╚═════╝╚═╝  ╚═╝╚══════╝   ╚═╝      ╚═╝  ╚═╝╚═╝     ╚═╝
    "));
    println!("{}", colored::Colorize::bright_green("    🎫 Ticket Management API Server"));
    println!("{}", colored::Colorize::yellow("    Version: 1.0.0"));
    println!();
}

#[tokio::main]
async fn main() {
    // Initialize logging first
    init_logging();
    
    // Print startup banner
    print_startup_banner();

    info!("🔧 Initializing application components...");

    // Create the ticket store
    let store = TicketStore::new();
    info!("📦 Ticket store initialized");

    let app_state = AppState { store };
    info!("🏗️  Application state configured");

    // Build the application with routes and middleware
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/tickets", post(create_ticket))
        .route("/tickets", get(list_tickets))
        .route("/tickets/:id", get(get_ticket))
        .route("/tickets/:id", patch(update_ticket))
        // Add request tracing middleware
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(|request: &axum::http::Request<_>, _span: &tracing::Span| {
                    info!(
                        method = %request.method(),
                        uri = %request.uri(),
                        headers = ?request.headers(),
                        "📨 Incoming request"
                    );
                })
                .on_response(|response: &axum::http::Response<_>, latency: std::time::Duration, _span: &tracing::Span| {
                    info!(
                        status = %response.status(),
                        latency = ?latency,
                        "📤 Response sent"
                    );
                }),
        )
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    info!("🛣️  Routes configured:");
    info!("   GET    /health");
    info!("   POST   /tickets");
    info!("   GET    /tickets");
    info!("   GET    /tickets/:id");
    info!("   PATCH  /tickets/:id");

    // Start the server
    let bind_address = "0.0.0.0:3000";
    info!("🌐 Binding server to {}", bind_address);

    match tokio::net::TcpListener::bind(bind_address).await {
        Ok(listener) => {
            let local_addr = listener.local_addr().unwrap();
            info!("✅ Server successfully bound to {}", local_addr);
            println!();
            println!("{}", colored::Colorize::bright_green("🚀 Server is running!"));
            println!("   📍 Local:    http://localhost:3000");
            println!("   📍 Network:  http://{}:3000", 
                     local_addr.ip().to_string().replace("0.0.0.0", "127.0.0.1"));
            println!("   📖 Health:   http://localhost:3000/health");
            println!();
            info!("🎯 Ready to accept connections");

            if let Err(e) = axum::serve(listener, app).await {
                tracing::error!("❌ Server error: {}", e);
            }
        }
        Err(e) => {
            tracing::error!("❌ Failed to bind server: {}", e);
            std::process::exit(1);
        }
    }

    warn!("🛑 Server shutting down");
}
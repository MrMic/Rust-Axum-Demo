use axum::{http::StatusCode, response::Html, routing::get, Router};
use tower_http::trace::{self, TraceLayer};
use tracing::{info, Level};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // construct a subscriber that prints formatted traces to stdout
    // let subscriber = tracing_subscriber::FmtSubscriber::new();

    // * INFO:  Start configuring a `fmt` subscriber
    tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        // .with_file(true)
        //.json()
        // Don't display the event's target (module path)
        .with_target(false)
        // Build the subscriber
        .init();

    // * INFO:  Start configuring a `Registry` subscriber
    /*
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    */

    // * INFO:  Start configuring a `appender` subscriber
    /*
    let info_file = rolling::daily("./logs", "info");

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_writer(info_file)
        .with_ansi(false)
        .init();
    */

    // * INFO: Build our new Router
    let app: axum::Router<()> = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/demo.html", get(get_demo_html))
        .route("/hello.html", get(hello_html))
        .route("/demo-status", get(demo_status))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    // ______________________________________________________________________
    /// axum handler for "GET /demo.html" which responds with HTML text.
    /// The `Html` type sets an HTTP header content-type of `text/html`.
    pub async fn get_demo_html() -> Html<&'static str> {
        "<h1>Hello!</h1>".into()
    }

    // ______________________________________________________________________
    /// axum handler that responds with typical HTML coming from a file.
    /// This uses the Rust macro `std::include_str` to include a UTF-8 file
    /// path, relative to `main.rs`, as a `&'static str` at compile time.
    pub async fn hello_html() -> Html<&'static str> {
        include_str!("./hello.html").into()
    }

    // ______________________________________________________________________
    /// axum handler for "GET /demo-status" which returns a HTTP status
    /// code, such as OK (200), and a custom user-visible string message.
    pub async fn demo_status() -> (StatusCode, String) {
        (StatusCode::OK, "Everything is OK".to_string())
    }

    // * INFO: Run our application as a hyper server on http://localhost:3001
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();
    info!("Listening on http://127.0.0.1:3001");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

use axum::{routing::get, Router};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // construct a subscriber that prints formatted traces to stdout
    // let subscriber = tracing_subscriber::FmtSubscriber::new();

    // * INFO:  Start configuring a `fmt` subscriber
    tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
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
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    // * INFO: Run our application as a hyper server on http://localhost:3001
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

use std::fmt::Write;
use std::{collections::HashMap, thread};

use axum::{
    extract::{Json, Path, Query},
    http::{header::CONTENT_TYPE, StatusCode, Uri},
    response::{AppendHeaders, Html, IntoResponse},
    routing::get,
    Router,
};

// use tracing_subscriber::fmt::format;

use base64::{engine::general_purpose, Engine};
use serde_json::{json, Value};
use tower_http::trace::{self, TraceLayer};
use tracing::{info, Level};

mod book;
use crate::book::Book;
mod data;
use data::DATA;

/// To access data, create a thread, spawn it, then get the lock.
/// When you're done, then join the thread with its parent thread.
// async fn print_data() {
//     thread::spawn(move || {
//         let data = data::DATA.lock().unwrap();
//         println!("data {:?}", data);
//     })
//     .join()
//     .unwrap();
// }

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // print_data().await;

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
        .route("/demo-uri", get(demo_uri))
        .route("/demo.png", get(get_demo_png))
        .route(
            "/foo",
            get(get_foo)
                .put(put_foo)
                .patch(patch_foo)
                .post(post_foo)
                .delete(delete_foo),
        )
        .route("/items/:id", get(get_items_id))
        .route("/items", get(get_items))
        .route("/demo.json", get(get_demo_json).put(put_demo_json))
        .route("/books", get(get_books).put(put_book))
        .route("/book/:id", get(get_book_id))
        .route("/books/:id/form", get(get_books_id_form))
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

    // ______________________________________________________________________
    /// axum handler for "GET /demo-uri" which shows the request's own URI.
    /// This shows how to write a handler that receives the URI.
    pub async fn demo_uri(uri: Uri) -> String {
        format!("The URI is: {:?}", uri)
    }

    /// axum handler for "GET /demo.png" which responds with an image PNG.
    /// This sets a header "image/png" then sends the decoded image data.
    async fn get_demo_png() -> impl IntoResponse {
        let png = concat!(
            "iVBORw0KGgoAAAANSUhEUgAAAAEAAAAB",
            "CAYAAAAfFcSJAAAADUlEQVR42mPk+89Q",
            "DwADvgGOSHzRgAAAAABJRU5ErkJggg=="
        );
        (
            ([(CONTENT_TYPE, "image/png")]),
            AppendHeaders([(CONTENT_TYPE, "image/png")]),
            general_purpose::STANDARD.decode(png).unwrap(),
        )
    }

    // ______________________________________________________________________
    /// axum handler for "GET /foo" which returns a string message.
    /// This shows our naming convention for HTTP GET handlers.
    pub async fn get_foo() -> String {
        "GET foo".to_string()
    }
    /// axum handler for "PUT /foo" which returns a string message.
    /// This shows our naming convention for HTTP PUT handlers.
    pub async fn put_foo() -> String {
        "PUT foo".to_string()
    }
    /// axum handler for "PATCH /foo" which returns a string message.
    /// This shows our naming convention for HTTP PATCH handlers.
    pub async fn patch_foo() -> String {
        "PATCH foo".to_string()
    }
    /// axum handler for "POST /foo" which returns a string message.
    /// This shows our naming convention for HTTP POST handlers.
    pub async fn post_foo() -> String {
        "POST foo".to_string()
    }
    /// axum handler for "DELETE /foo" which returns a string message.
    /// This shows our naming convention for HTTP DELETE handlers.
    pub async fn delete_foo() -> String {
        "DELETE foo".to_string()
    }

    // ______________________________________________________________________
    /// axum handler for "GET /items/:id" which returns a string message.
    /// This extract a path parameter the deserialize it as needed.
    pub async fn get_items_id(Path(id): Path<String>) -> String {
        format!("Get items with path id:  {:?}", id)
    }

    // ______________________________________________________________________
    pub async fn get_items(Query(params): Query<HashMap<String, String>>) -> String {
        format!("Get items with  query params: {:?}", params)
    }

    // ______________________________________________________________________
    /// axum handler for "PUT /demo.json" which uses `aumx::extract::Json`.
    /// This buffers the request body then deserializes it bu using serde.
    /// The `Json` type supports types that implement `serde::Deserialize`.
    pub async fn get_demo_json() -> Json<Value> {
        json!({ "a": "b"}).into()
    }

    // ______________________________________________________________________
    /// axum handler for "PUT /demo.json" which uses `aumx::extract::Json`.
    /// This buffers the request body then deserializes it using serde.
    /// The `Json` type supports types that implement `serde::Deserialize`.
    pub async fn put_demo_json(Json(data): Json<Value>) -> String {
        format!("PUT demo JSON data: {:?}", data)
    }

    // _____________________________________________________________________
    /// axum handler for "GET /books" which responds with a resource page.
    /// This demo uses our DATA; a production app could use a database.
    /// This demo must clone the DATA in order to sort items by title.
    pub async fn get_books() -> axum::response::Html<String> {
        thread::spawn(move || {
            let data = DATA.lock().unwrap();
            let mut books = data.values().collect::<Vec<_>>().clone();
            books.sort_by(|a, b| a.title.cmp(&b.title));

            let mut result = String::new();
            books.iter().for_each(|&book| {
                writeln!(result, "<p>{}</p>", book).unwrap();
            });
            result
        })
        .join()
        .unwrap()
        .into()
    }

    // _____________________________________________________________________
    /// axum handler for "GET /books/:id" which responds with one resource Html
    /// This demo app uses our DATA variable, and iterates on it to find the i
    /// th book. This demo must clone the DATA in order to sort items by title.
    pub async fn get_book_id(Path(id): Path<u32>) -> axum::response::Html<String> {
        thread::spawn(move || {
            let data = DATA.lock().unwrap();
            match data.get(&id) {
                Some(book) => {
                    let mut result = String::new();
                    writeln!(result, "<p>{}</p>", book).unwrap();
                    result
                }
                None => {
                    let mut result = String::new();
                    writeln!(result, "<p>Book not found</p>").unwrap();
                    result
                }
            }
        })
        .join()
        .unwrap()
        .into()
    }

    // _____________________________________________________________________
    /// axum handler for "PUT /books" which creates a new book resource.
    /// This demo shows how axum can extract JSON data into a Book struct.
    pub async fn put_book(
        Json(book): Json<Book>,
    ) -> axum::response::Html<String> {
        thread::spawn(move || {
            let mut data = DATA.lock().unwrap();
            data.insert(book.id, book.clone());

            let mut result = String::new();
            writeln!(result, "PUT book: {}", book).unwrap();
            result
        })
        .join()
        .unwrap()
        .into()
    }

    // _____________________________________________________________________
    /// axum handler for "GET /books/:id/form" which responds with a form.
    /// This demo shows how to write a typical HTML form with input fields.
    pub async fn get_books_id_form(Path(id): Path<u32>) -> axum::response::Html<String> {
        thread::spawn(move || {
            let data = DATA.lock().unwrap();
            match data.get(&id) {
                Some(book) => format!(
                    concat!(
                        "<form method=\"post\" action=\"/books/{}/form\">\n",
                        "<input type=\"hidden\" name=\"id\" value=\"{}\">\n",
                        "<p><input name=\"title\" value=\"{}\"></p>\n",
                        "<p><input name=\"author\" value=\"{}\"></p>\n",
                        "<input type=\"submit\" value=\"Save\">\n",
                        "</form>\n"
                    ),
                    &book.id, &book.id, &book.title, &book.author
                ),
                None => format!("<p>Book id {} not found</p>", id),
            }
        })
        .join()
        .unwrap()
        .into()
    }

    // ══════════════════════════════════════════════════════════════════════
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new httpc test client with a base URL (will be prefixed for all calls)
    // The client will have a cookie_store.
    let hc = httpc_test::new_client("http://127.0.0.1:3001")?;

    //// do_get, do_post, do_put, do_patch, do_delete return a httpc_test::Response
    // Simple do_get
    // httpc_test::Response
    let res = hc.do_get("/").await?;
    // let _status = res.status();
    // Pretty print the result (status, headers, response cookies, client cookies, body)
    res.print().await?;

    // GET Request to HTML
    let res = hc.do_get("/demo.html").await?;
    // let _status = res.status();
    res.print().await?;

    // GET Request to HTML
    let res = hc.do_get("/hello.html").await?;
    res.print().await?;

    // GET Request to HTML
    let res = hc.do_get("/demo-status").await?;
    res.print().await?;

    Ok(())
}

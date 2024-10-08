use serde_json::json;

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

    // ______________________________________________________________________
    // GET Request to HTML
    let res = hc.do_get("/demo.html").await?;
    // let _status = res.status();
    res.print().await?;

    // ______________________________________________________________________
    // GET Request to HTML
    let res = hc.do_get("/hello.html").await?;
    res.print().await?;

    // ______________________________________________________________________
    // GET Request to HTML
    let res = hc.do_get("/demo-status").await?;
    res.print().await?;

    // ______________________________________________________________________
    // GET Request to HTML
    let res = hc.do_get("/demo-uri").await?;
    res.print().await?;

    // ______________________________________________________________________
    // PNG image
    let res = hc.do_get("/demo.png").await?;
    res.print().await?;

    // ______________________________________________________________________
    // GET "/foo"
    let res = hc.do_get("/foo").await?;
    res.print().await?;
    // PUT "/foo"
    let res = hc.do_put("/foo", "").await?;
    res.print().await?;
    // PATCH "/foo"
    let res = hc.do_patch("/foo", "").await?;
    res.print().await?;
    // POST "/foo"
    let res = hc.do_post("/foo", "").await?;
    res.print().await?;
    // DELETE "/foo"
    let res = hc.do_delete("/foo").await?;
    res.print().await?;

    //______________________________________________________________________
    // GET id
    let res = hc.do_get("/items/1").await?;
    res.print().await?;

    // ______________________________________________________________________
    // GET items
    let res = hc.do_get("/items?a=b").await?;
    res.print().await?;

    //______________________________________________________________________
    // GET Json
    let res = hc.do_get("/demo.json").await?;
    res.print().await?;

    //______________________________________________________________________
    // PUT Json
    let res = hc.do_put("/demo.json", json!({"hello": "world"})).await?;
    res.print().await?;

    //______________________________________________________________________
    // GET books
    let res = hc.do_get("/books").await?;
    res.print().await?;

    //______________________________________________________________________
    // GET book 1
    let res = hc.do_get("/book/1").await?;
    res.print().await?;

    //______________________________________________________________________
    // PUT book 4
    let res = hc
        .do_put(
            "/books",
            json!({
                "id": 4,
                "title": "Decameron",
                "author": "Giovanni Boccaccio"
            }),
        )
        .await?;
    res.print().await?;

    //______________________________________________________________________
    // GET book 1
    let res = hc.do_get("/books/1/form").await?;
    res.print().await?;

    //______________________________________________________________________
    // POST book 4
    /*
    let res = hc
        .do_post(
            "/books/1/form",
            (
                r#"{
                    "id": 4,
                    "title": "Another Title",
                    "author": " Someone Else"
                }
                "#,
                "application/x-www-form-urlencoded",
            ),
        )
        .await?;
    res.print().await?;
    */

    //______________________________________________________________________
    // DELETE book 1
    let res = hc.do_delete("/book/1").await?;
    res.print().await?;
    let res = hc.do_get("/books").await?;
    res.print().await?;

    // ______________________________________________________________________
    Ok(())
}

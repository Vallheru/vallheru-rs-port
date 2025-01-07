use anyhow::Result;
use serde_json::json;

async fn valid_login() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3004")?;

    // hc.do_get("/api/hello").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!(vallheru::api::LoginRequest {
            email: String::from("admin@vallheru.pl"),
            password: String::from("admin"),
        }),
    );
    req_login.await?.print().await?;

    Ok(())
}

async fn invalid_login() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3004")?;

    // hc.do_get("/api/hello").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!(vallheru::api::LoginRequest {
            email: String::from("admin@vallheru.pl"),
            password: String::from("invalid_pass"),
        }),
    );
    req_login.await?.print().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    valid_login().await?;
    invalid_login().await?;

    Ok(())
}

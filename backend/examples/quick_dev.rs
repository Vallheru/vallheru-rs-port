use anyhow::Result;
use serde_json::json;
use vallheru::api::login;

async fn valid_login() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3004")?;

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

async fn not_found_fallback() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3004")?;

    let req_login = hc.do_get("/invalid-page");
    req_login.await?.print().await?;

    Ok(())
}

async fn protected_endpoint() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3004")?;
    let hc = hc.reqwest_client();

    let res = hc
        .get("http://localhost:3004/api/game")
        .header(
            "Authorization",
            "Token 15e60c90142c844c2ae8d0cd4ce5390baec05d528cb894e57b4666a335f2b0383",
        )
        .send()
        .await?;

    println!("{}", res.text().await?);

    Ok(())
}

async fn login() -> vallheru::api::Result<vallheru::api::LoginResponse> {
    vallheru::api::api_request(
        None,
        "http://localhost:3004",
        &vallheru::api::LoginRequest {
            email: String::from("admin@vallheru.pl"),
            password: String::from("admin"),
        },
        None,
    )
    .await
}

async fn get_player(token: &str) -> vallheru::api::Result<vallheru::api::PlayerResponse> {
    vallheru::api::api_request(
        None,
        "http://localhost:3004",
        &vallheru::api::PlayerRequest {
            identifier: vallheru::api::player::PlayerIdentifier::AuthToken,
        },
        Some(token),
    )
    .await
}

async fn test_player_api_endpoint() {
    let login_resp = login().await.unwrap();

    let player = get_player(&login_resp.token).await.unwrap().player;
    assert_eq!(1, player.id);
}

#[tokio::main]
async fn main() -> Result<()> {
    // valid_login().await?;
    // invalid_login().await?;
    // not_found_fallback().await?;

    // protected_endpoint().await?;
    test_player_api_endpoint().await;
    Ok(())
}

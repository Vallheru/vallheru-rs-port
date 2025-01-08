use serde_json::json;

const API_URL: &str = "http://nebula-dev.local.mainnet.community:3004";

pub async fn login(email: &str, password: &str) -> Result<vallheru::api::LoginResponse, ()> {
    let client = reqwest::Client::new();

    let req = vallheru::api::LoginRequest {
        email: String::from(email),
        password: String::from(password),
    };

    let res = client
        .post(String::from(API_URL) + "/api/login")
        .json(&req)
        .send()
        .await
        .unwrap();

    let text = res.text().await.unwrap();
    let res: vallheru::api::LoginResponse = serde_json::from_str(&text).unwrap();

    Ok(res)
}

#[derive(serde::Deserialize)]
struct Healthcheck {
    ok: bool,
}

#[tokio::main]
async fn main() {
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let port: u16 = port.parse().expect("SERVER_PORT must be a number");

    let url = format!("http://localhost:{port}/_health");
    let res: Healthcheck = reqwest::get(&url)
        .await
        .expect("failed to make request")
        .json()
        .await
        .expect("failed to parse json");
    assert!(res.ok, "healthcheck failed");
}

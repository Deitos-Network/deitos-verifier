use reqwest::Error as ReqwestError;
use warp::{http::Response, Filter, Rejection, Reply, http::StatusCode};
use sha2::{Sha256, Digest};
use once_cell::sync::Lazy;
mod config;
use crate::config::Config;

static CONFIG: Lazy<Config> = Lazy::new(Config::new);

async fn fetch_file_content(file_path: String) -> Result<Vec<u8>, ReqwestError> {
    println!("CONFIG.hdfs_uri {}",CONFIG.hdfs_uri);
    let url = format!("{}/{}?op=OPEN", CONFIG.hdfs_uri,file_path );
    let client = reqwest::Client::new();
    let res = client.get(&url).send().await?;
    let bytes = res.bytes().await?;
    Ok(bytes.to_vec())
}

fn calculate_sha256_checksum(file_content: Vec<u8>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&file_content);
    let result = hasher.finalize();
    format!("{:x}", result)
}

async fn handle_request(file_path: String) -> Result<impl Reply, Rejection> {
    println!("Incoming request");
    match fetch_file_content(file_path).await {
        Ok(content) => {
            let checksum = calculate_sha256_checksum(content);
            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(checksum)
                .unwrap())
        },
        Err(e) => {
            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(e.to_string())
                .unwrap())
        },
    }
}

#[tokio::main]
async fn main() {
    let calculate_route = warp::path!("calculate" / String)
        .and(warp::get())
        .and_then(handle_request);
   println!("Listening on port {}", CONFIG.port);
    warp::serve(calculate_route)
        .run(([127, 0, 0, 1], CONFIG.port))
        .await;
}

use std::collections::HashMap;

use chrono::NaiveDate;
use serde::Deserialize;


type HttpClientResult<T> = Result<T, Box<dyn std::error::Error>>;
pub const COTACAO_INICIAL: f64 = 5.30;

#[derive(Deserialize, Debug)]
struct ApiLayerResponse {
    base: String, 
    date: NaiveDate,
    rates: HashMap<String, f64>,
    success: bool,
    timestamp: usize,
}

async fn fetch_dollar_quotation(url: &str, key: &str) -> HttpClientResult<ApiLayerResponse> {
    let client = reqwest::Client::new();
    let resp = client.get(url)
        .header("apikey", key)
        .send()
        .await?
        .json::<ApiLayerResponse>()
        .await?;
    Ok(resp)
}

pub async fn get_dollar_quotation(url: &str, key: &str) -> Option<f64> {
    if let Ok(res) = fetch_dollar_quotation(url, key).await {
        res.rates.get("BRL").copied()
    } else {
        None
    }
}
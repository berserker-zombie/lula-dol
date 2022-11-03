use std::collections::HashMap;
use chrono::{NaiveDate};
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ApiLayerResponse {
    base: String, 
    date: NaiveDate,
    rates: HashMap<String, f64>,
    success: bool,
    timestamp: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let bot_token = std::env::var("TELOXIDE_TOKEN").expect("Token do BOT deve ser fornecida!");
    let api_key = std::env::var("API_KEY").expect("Chave da API deve ser fornecida!");

    let client = reqwest::Client::new();
    let resp = client.get("https://api.apilayer.com/exchangerates_data/latest?symbols=BRL&base=USD")
        .header("apikey", api_key)
        .send()
        .await?
        .json::<ApiLayerResponse>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
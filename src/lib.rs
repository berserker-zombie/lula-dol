mod client;
mod bot;
mod database;
use dotenv::dotenv;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let _bot_token = std::env::var("TELOXIDE_TOKEN").expect("Token do BOT deve ser fornecida!");
    let api_key = std::env::var("API_KEY").expect("Chave da API deve ser fornecida!");
    let api_url = std::env::var("API_URL").expect("URL da API deve ser fornecida!");
    bot::start_bot(&api_key, &api_url).await;
    Ok(())
}
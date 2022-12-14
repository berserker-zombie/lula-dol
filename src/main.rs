use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let _bot_token = std::env::var("TELOXIDE_TOKEN").expect("Token do BOT deve ser fornecida!");
    let _api_key = std::env::var("API_KEY").expect("Chave da API deve ser fornecida!");
    let _api_url = std::env::var("API_URL").expect("URL da API deve ser fornecida!");
    lula_dol::run().await
}
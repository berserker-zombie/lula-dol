use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let bot_token = std::env::var("TELOXIDE_TOKEN").expect("Token do BOT deve ser fornecida!");
    let api_key = std::env::var("API_KEY").expect("Chave da API deve ser fornecida!");
    println!("{bot_token:?} - {api_key:?}");
    Ok(())
}
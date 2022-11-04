use std::{time::{Instant, Duration}};
use once_cell::sync::Lazy;
use teloxide::{prelude::*, utils::command::BotCommands};
use tokio::sync::Mutex;
use crate::client;


#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Start,
    #[command(description = "display this text.")]
    Help,
}

static DAY: u64 = 60 * 60 * 24;
static TIME: u64 = 15;

struct QuotationMemoryInfo {
    quotation: f64,
    instant: Instant,
}

impl QuotationMemoryInfo {
    fn new(quotation: f64, instant: Instant) -> Self {
        Self { quotation, instant }
    }
}

pub async fn start_bot() {
    static MEM: Lazy<Mutex<QuotationMemoryInfo>> = Lazy::new(|| Mutex::new(QuotationMemoryInfo::new(7.0, Instant::now())));
    let bot = Bot::from_env();
    teloxide::repl(bot, handle).await;
    // Command::repl(bot, answer).await;
}

async fn handle(bot: Bot, msg: Message) -> ResponseResult<()> {
    static MEM: Lazy<Mutex<QuotationMemoryInfo>> = Lazy::new(|| Mutex::new(QuotationMemoryInfo::new(7.0, Instant::now())));
    // Verifica se passou o tempo
    println!("{}", msg.chat.id);
    let mut mem = MEM.lock().await;
    if mem.instant + Duration::new(TIME, 0) >= Instant::now() {
        println!("ID: 1, Passando por aqui!");
        return Ok(());
    }
    println!("ID: 0, Passando por aqui!");
    mem.instant = Instant::now();
    mem.quotation -= 0.10;
    bot.send_message(msg.chat.id, format!("O Dólar está valendo R${}, companheiro!", mem.quotation)).await?;
    Ok(())
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => bot.send_message(msg.chat.id, format!("Tá na hora de salvar a democracia, companheiro!")).await?,
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
    };
    Ok(())
}
use chrono::{Utc, Duration};
use teloxide::{prelude::*, utils::command::BotCommands, dispatching::UpdateFilterExt};
use tokio::sync::Mutex;
use std::{sync::Arc};
use crate::{client, database};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Inicializa o LulaDol.")]
    Start,
    #[command(description = "Mostra as opções do LulaDol.")]
    Help,
    #[command(description = "Mostra a cotação mais recente do Dolar no Governo Lula.")]
    Dolar,
}

#[derive(Clone)]
struct ApiInfo {
    pub key: String,
    pub url: String,
}

const DOL_START: f64 = 5.30;
const INTERVAL: i64 = 60 * 60 * 24; // 20 horas

pub async fn start_bot(key: &str, url: &str) {
    let bot = Bot::from_env();    
    let mem = match database::read_info() {
        Ok(info) => info,
        Err(_) => {
            eprintln!("Falha ao carregar dados do banco de dados local. Consultando cotação via API");
            let instant = Utc::now();
            let quotation = client::get_dollar_quotation(&url, &key)
                .await
                .unwrap_or(DOL_START);
            _ = database::create_info(database::Memory{quotation, instant});
            database::Memory{quotation, instant}
        },
    };
    let shared_mem = Arc::new(Mutex::new(mem));
    let api_info = ApiInfo{key: key.to_string(), url: url.to_string()};

    let handler = Update::filter_message()
        .branch(
        dptree::entry()
            .filter_command::<Command>()
            .endpoint(commands_handler),
        );
    
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![shared_mem, api_info])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn commands_handler(
    shared_mem: Arc<Mutex<database::Memory>>,
    api_info: ApiInfo,
    bot: Bot,
    msg: Message,
    cmd: Command,
) -> Result<(), teloxide::RequestError> {
    let text = match cmd {
        Command::Start => { format!("Tá na hora de salvar a democracia, companheiro!") },
        Command::Help => { format!("Os comandos disponíveis são: \n/start Pra inicializar o comunismo\n/dolar Pra saber o quanto o partido dos trabalhadores já melhorou a economia\nSe precisar de ajuda, mande um /help") },
        Command::Dolar => {
            let now = Utc::now();
            let mut mem = shared_mem.lock().await;
            println!("{:?}", mem);
            if now.signed_duration_since(mem.instant) >= Duration::seconds(INTERVAL) {
                println!("Atualizando cotação do dolar via API");
                if let Some(quotation) = client::get_dollar_quotation(&api_info.url, &api_info.key).await {
                    match database::create_info(database::Memory::new(quotation, now)) {
                        Ok(()) => (),
                        Err(_) => eprintln!("Não foi possível salvar a cotação no banco de dados. Seguiremos com o processamento.")
                    };
                    mem.quotation = quotation;
                    mem.instant = now;
                } else {
                    eprintln!("Falha ao consultar a cotação na API.");
                }
            }
            let percentage = (DOL_START - mem.quotation) / DOL_START * 100.0;
            if percentage >= 0.0 {
                format!("A cotação do dolar hoje é R${:.2}.\nMeu governo já melhorou o câmbio em {:.2}%.", mem.quotation, percentage.abs())
            } else {
                format!("A cotação do dolar hoje é R${:.2}.\nDiante da ameaça fascista o câmbio piorou em {:.2}%.", mem.quotation, percentage.abs())
            }
        },
    };
    bot.send_message(msg.chat.id, text).await?;
    Ok(())
}
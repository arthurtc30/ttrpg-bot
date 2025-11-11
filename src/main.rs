use poise::serenity_prelude as serenity;
use anyhow::Context as _;
use std::env;
use std::collections::HashMap;
use tokio::sync::{RwLock, Mutex};
use poise::serenity_prelude::GuildId;
use rand::rngs::StdRng;
use rand::{SeedableRng};

mod commands;
mod localization;

pub struct Data {
    language_map: RwLock<HashMap<GuildId, localization::Language>>,
    rng: Mutex<StdRng>,
}

type Context<'a> = poise::Context<'a, Data, anyhow::Error>;
type Error = anyhow::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok(); 

    let token = env::var("DISCORD_TOKEN")
        .context("DISCORD_TOKEN não encontrado no arquivo .env")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::roll::roll(),
                commands::config::config(),
                commands::help::help(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                let debug_server_id = env::var("DEBUG_SERVER_ID");
                let commands = &framework.options().commands;

                if let Ok(id_string) = debug_server_id {
                    let guild_id_u64 = id_string
                        .parse::<u64>()
                        .context("DEBUG_SERVER_ID deve ser um número (u64) válido")?;

                    let guild_id = serenity::GuildId::new(guild_id_u64);

                    println!(
                        "Iniciando em MODO DEBUG. Registrando no servidor (Guild): {}", 
                        guild_id
                    );

                    let empty_commands: &[poise::Command<Data, Error>] = &[];
                    poise::builtins::register_globally(ctx, empty_commands).await?;
                    poise::builtins::register_in_guild(ctx, commands, guild_id).await?;
                    println!("Comandos de Guild registrados!");
                } else {
                    println!("Iniciando em MODO PRODUÇÃO. DEBUG_SERVER_ID não definida. Registrando globalmente.");
                    poise::builtins::register_globally(ctx, commands).await?;
                    println!("Comandos globais registrados.");
                }

                println!("Bot conectado como {}!", _ready.user.name);

                println!("Gerando semente de aleatoriedade (pode demorar um pouco)...");
                let mut seeder_rng = rand::thread_rng();
                let rng = StdRng::from_rng(&mut seeder_rng);
                println!("Semente gerada.");

                Ok(Data {
                    language_map: RwLock::new(HashMap::new()),
                    rng: Mutex::new(rng),
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(
        token,
        serenity::GatewayIntents::non_privileged(),
    )
    .framework(framework)
    .await;

    client.unwrap().start().await.unwrap();

    Ok(())
}
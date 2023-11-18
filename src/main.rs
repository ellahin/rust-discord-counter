mod commands;

use dotenvy::dotenv;
use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use sqlx::migrate::{MigrateDatabase, Migrator};
use sqlx::Sqlite;
use sqlx::{Pool, SqlitePool};
use std::env;
use std::path::Path;

struct Handler {
    database: Pool<Sqlite>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let database_clone = self.database.clone();
        tokio::spawn(async move {
            if let Interaction::ApplicationCommand(command) = interaction {
                let content = match command.data.name.as_str() {
                    "ping" => commands::ping::run(&command.data.options),
                    "updateitem" => {
                        commands::updateitem::run(
                            command.data.options.clone().as_ref(),
                            database_clone.clone(),
                            command.guild_id.clone(),
                        )
                        .await
                    }
                    "addcount" => {
                        commands::addcount::run(
                            command.data.options.clone().as_ref(),
                            database_clone.clone(),
                            command.guild_id.clone(),
                            command.user.clone(),
                        )
                        .await
                    }
                    "showcount" => {
                        commands::showcount::run(
                            command.data.options.clone().as_ref(),
                            database_clone.clone(),
                            command.guild_id.clone(),
                            command.user.clone(),
                        )
                        .await
                    }
                    _ => "not implemented :(".to_string(),
                };

                if let Err(why) = command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| message.content(content))
                    })
                    .await
                {
                    println!("Cannot respond to slash command: {}", why);
                }
            }
        });
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let _ = Command::create_global_application_command(&ctx.http, |command| {
            commands::ping::register(command)
        })
        .await;
        let _ = Command::create_global_application_command(&ctx.http, |command| {
            commands::updateitem::register(command)
        })
        .await;

        let _ = Command::create_global_application_command(&ctx.http, |command| {
            commands::addcount::register(command)
        })
        .await;

        let _ = Command::create_global_application_command(&ctx.http, |command| {
            commands::showcount::register(command)
        })
        .await;
    }
}
#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv().expect(".env file not found");

    if env::var("DATABASE_URL").is_err() {
        panic!("DATABASE_URL not in environment vars");
    }

    if env::var("DISCORD_TOKEN").is_err() {
        panic!("DISCORD_TOKEN not in environment vars");
    }

    if env::var("DISOORD_PERMISSION").is_err() {
        panic!("DISOORD_PERMISSION not in environment vars");
    }

    let database_url = env::var("DATABASE_URL").unwrap();

    if !sqlx::Sqlite::database_exists(&database_url).await.unwrap() {
        sqlx::Sqlite::create_database(&database_url).await.unwrap();
    }

    let migration_path = Path::new("./migrations");

    let sql_pool = SqlitePool::connect(&database_url).await.unwrap();

    Migrator::new(migration_path)
        .await
        .unwrap()
        .run(&sql_pool)
        .await
        .unwrap();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let handler = Handler { database: sql_pool };

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

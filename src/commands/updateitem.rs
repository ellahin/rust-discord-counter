use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::model::prelude::GuildId;

use sqlx::sqlite::SqlitePool;
use sqlx::Error;

pub async fn run(
    options: &[CommandDataOption],
    database: SqlitePool,
    guild_option: Option<GuildId>,
) -> String {
    if guild_option.is_none() {
        return "Error No GuildId".to_string();
    }

    let guildid: i64 = i64::try_from(guild_option.unwrap().as_u64().clone()).unwrap();
    let row_query_raw = sqlx::query!(
        r#"
        SELECT realmID 
        from servers 
        where realmID = ?1
        "#,
        guildid
    )
    .fetch_optional(&database)
    .await;

    if row_query_raw.is_err() {
        return "Database Error".to_string();
    }

    let option = options
        .get(0)
        .expect("Expected item option")
        .resolved
        .as_ref()
        .expect("Expected Item Option");

    let CommandDataOptionValue::String(item) = option else {
        panic!("Cannot access option")
    };

    let row_query = row_query_raw.unwrap();

    if row_query.is_none() {
        let put_query = sqlx::query!(
            r#"
        INSERT INTO servers(realmID, item)
        VALUES(?1, ?2)
        "#,
            guildid,
            item
        )
        .execute(&database)
        .await;

        if put_query.is_err() {
            return "Error cannot put into database".to_string();
        }

        let update_query = sqlx::query!(
            "UPDATE servers SET item = ?1 WHERE realmID = ?2",
            item,
            guildid,
        )
        .execute(&database)
        .await;

        if update_query.is_err() {
            return "Database Error".to_string();
        }

        return format!("Item set to {}", item).to_string();
    }

    let update_query = sqlx::query!(
        "UPDATE servers SET item = ?1 WHERE realmID = ?2",
        item,
        guildid,
    )
    .execute(&database)
    .await;

    if update_query.is_err() {
        return "Database Error".to_string();
    }
    return format!("Item set to {}", item).to_string();
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("updateitem")
        .description("Updates what is being counted")
        .create_option(|option| {
            option
                .name("item")
                .description("Name that item should be updated too")
                .kind(CommandOptionType::String)
                .min_length(1)
                .max_length(255)
                .required(true)
        })
}

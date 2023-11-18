use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::prelude::GuildId;

use serenity::model::user::User;
use sqlx::sqlite::SqlitePool;

pub async fn run(
    options: &[CommandDataOption],
    database: SqlitePool,
    guild_option: Option<GuildId>,
    user: User,
) -> String {
    if guild_option.is_none() {
        return "Error No GuildId".to_string();
    }

    let guildid: i64 = i64::try_from(guild_option.unwrap().as_u64().clone()).unwrap();

    let userid: i64 = i64::try_from(user.id.as_u64().clone()).unwrap();

    let get_user_raw = sqlx::query!(
        "SELECT * from usercounts where realmID = ?1 and userID = ?2",
        guildid,
        userid
    )
    .fetch_optional(&database)
    .await;

    if get_user_raw.is_err() {
        return "Database Error".to_string();
    }

    let get_user = get_user_raw.unwrap();

    if get_user.is_none() {
        return create_user(guildid, userid, database).await;
    }

    let guild_fetch_raw = sqlx::query!("SELECT item from servers where realmID = ?1", guildid)
        .fetch_one(&database)
        .await;

    if guild_fetch_raw.is_err() {
        return "Database Error".to_string();
    }

    let guild_fetch = guild_fetch_raw.unwrap();

    let user_row = get_user.unwrap();

    let new_count = user_row.count + 1;

    let update_user = sqlx::query!(
        "UPDATE usercounts set count = ?1 where ID = ?2",
        new_count,
        user_row.ID
    )
    .execute(&database)
    .await;

    if update_user.is_err() {
        return "Database Error".to_string();
    }

    return format!("You have {} {}", new_count, guild_fetch.item).to_string();
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("addcount").description("Add a count")
}

async fn create_user(guildid: i64, userid: i64, database: SqlitePool) -> String {
    let guild_fetch_raw = sqlx::query!(
        "SELECT realmID, item from servers where realmID = ?1",
        guildid
    )
    .fetch_optional(&database)
    .await;

    if guild_fetch_raw.is_err() {
        return "Databes Error".to_string();
    }

    let guild_fetch = guild_fetch_raw.unwrap();

    if guild_fetch.is_none() {
        let create_guild = sqlx::query!(
            r#"
        INSERT INTO servers(realmID, item)
        VALUES(?1, ?2)
        "#,
            guildid,
            "Place Holder"
        )
        .execute(&database)
        .await;

        if create_guild.is_err() {
            return "Database Error".to_string();
        }
    }

    let create_user = sqlx::query!(
        "INSERT INTO usercounts(realmID, userID, count) VALUES(?1, ?2, ?3)",
        guildid,
        userid,
        1
    )
    .execute(&database)
    .await;

    if create_user.is_err() {
        println!("databes error: {:#?}", create_user);
        return "Database Error making user".to_string();
    }

    if guild_fetch.is_none() {
        return "You have 1 Place Holder, to update the item name run /updateitem".to_string();
    }

    return format!("You have 1 {}", guild_fetch.unwrap().item).to_string();
}

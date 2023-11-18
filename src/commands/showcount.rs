use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::model::prelude::GuildId;
use serenity::model::user::User;
use sqlx::sqlite::SqlitePool;

pub async fn run(
    options: &[CommandDataOption],
    database: SqlitePool,
    guild_option: Option<GuildId>,
    user: User,
) -> String {
    let mut check_user = user;

    if options.get(0).is_some() {
        let option = options
            .get(0)
            .expect("Expected item option")
            .resolved
            .as_ref()
            .expect("Expected Item Option");

        let CommandDataOptionValue::User(item, _member) = option else {
            panic!("could not get user")
        };

        check_user = item.clone();
    }

    if guild_option.is_none() {
        return "Error no Guild id".to_string();
    }

    let guild = guild_option.unwrap();

    let guildid: i64 = i64::try_from(guild.as_u64().clone()).unwrap();
    let userid: i64 = i64::try_from(check_user.id.as_u64().clone()).unwrap();
    let user_query_raw = sqlx::query!(
        "SELECT * from usercounts where realmID = ?1 and userID = ?2",
        guildid,
        userid
    )
    .fetch_optional(&database)
    .await;

    if user_query_raw.is_err() {
        return "Database Error".to_string();
    }

    let user_query_option = user_query_raw.unwrap();

    if user_query_option.is_none() {
        return format!(
            "There is no count for {}, to start the count run /addcount",
            check_user.name
        );
    }

    let guild_query_raw = sqlx::query!("SELECT * from servers where realmID = ?1", guildid)
        .fetch_one(&database)
        .await;

    if guild_query_raw.is_err() {
        return "Database Error".to_string();
    }

    let guild_query = guild_query_raw.unwrap();

    return format!(
        "{} has {} {}",
        check_user.name,
        user_query_option.unwrap().count,
        guild_query.item
    );
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("showcount")
        .description("Shows the current count of a user")
        .create_option(|option| {
            option
                .name("user")
                .description("User to check (Optional)")
                .kind(CommandOptionType::User)
                .required(false)
        })
}

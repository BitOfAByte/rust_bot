use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
async fn invite(ctx: &Context, msg: &Message) -> CommandResult {

    msg.reply(ctx, "https://discord.com/api/oauth2/authorize?client_id=1176593884801929226&permissions=8&scope=bot").await?;


    Ok(())
}



use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
};
use serenity::model::channel::ReactionType;

#[command]
async fn react(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let message_id = args.single::<u64>()?;

    if message_id == 0 {
        msg.channel_id.say(&ctx.http, "Please provide a message id").await?;
        return Ok(());
    }

    msg.channel_id
        .message(&ctx.http, message_id)
        .await?
        .react(&ctx.http, ReactionType::Unicode("👍".to_string()))
        .await?;
    Ok(())
}

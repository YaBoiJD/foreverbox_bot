// Serenity
use serenity::{
    client::bridge::gateway::ShardId,
    framework::standard::{
        CommandResult,
        macros::command,
    },
    model::prelude::*,
    prelude::*,
};

// ShardManager
use crate::ShardManagerContainer;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(v) => v,
        None => {
            msg.reply(ctx, "There was a problem getting the shard manager").await?;

            return Ok(());
        },
    };

    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;

    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            msg.reply(ctx, "No shard found").await?;

            return Ok(());
        },
    };

    if let Some(latency) = runner.latency {
        msg.channel_id.say(ctx, &format!("The shard latency is {:?}", latency)).await?;

        return Ok(());
    } else {
        msg.channel_id.say(ctx, "Cannot get latency").await?;

        return Ok(());
    }
}

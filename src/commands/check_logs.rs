use crate::log_upload::check_for_logs;

use super::{Context, Error};
use poise::CreateReply;
use serenity::all::Message;

#[poise::command(
    track_edits,
    context_menu_command = "Check for logs",
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn check_logs_normal(ctx: Context<'_>, msg: Message) -> Result<(), Error> {
    check_logs(ctx, msg, false).await
}

#[poise::command(
    track_edits,
    context_menu_command = "Check for logs (Compact)",
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn check_logs_compact(ctx: Context<'_>, msg: Message) -> Result<(), Error> {
    check_logs(ctx, msg, true).await
}

async fn check_logs(ctx: Context<'_>, msg: Message, compact: bool) -> Result<(), Error> {
    let reply = ctx.reply("Scanning for logs...").await?;

    match check_for_logs(ctx.serenity_context(), &msg, true, compact).await {
        Ok(Some(edit)) => {
            let mut reply_builder = CreateReply::default().content(edit.0).components(edit.2);
            for ele in edit.1 {
                reply_builder = reply_builder.embed(ele);
            }

            reply.edit(ctx, reply_builder).await?;
        }
        Ok(None) => {
            reply
                .edit(ctx, CreateReply::default().content("No logs found."))
                .await?;
        }
        Err(err) => {
            reply
                .edit(
                    ctx,
                    CreateReply::default().content(format!("Error, cannot upload logs: {err}")),
                )
                .await?;
        }
    };
    Ok(())
}

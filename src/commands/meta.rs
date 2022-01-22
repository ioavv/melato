use serenity::framework::standard::{macros::{command, help}, 
    CommandResult, help_commands,
    Args,
    HelpOptions,
    CommandGroup};
    
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::env;
use std::collections::HashSet;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let data = env::var("APPDATA")?;
    msg.channel_id.say(&ctx.http, data).await?;

    Ok(())
}


#[help]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
 ) -> CommandResult {
     let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
     Ok(())
 } 
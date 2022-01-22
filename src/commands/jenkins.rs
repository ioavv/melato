use serenity::framework::standard::{macros::{command}, 
    CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::env;
use reqwest::{header};

#[command]
#[aliases("b")]
#[description = "Build"]
async fn build(ctx: &Context, msg: &Message) -> CommandResult {
    let mut headers = header::HeaderMap::new();
    let mut token = header::HeaderValue::from_str(&format!("Basic {}",
    env::var("JENKINS_AUTH").expect("missing jenkins token"))).expect("creating token failed");
    token.set_sensitive(true);
    headers.insert("Authorization", token);
    let client = reqwest::Client::builder()
    .default_headers(headers)
    .build()
    .expect("creating client failed");
    let result = client.post(
        env::var("JENKINS_URL").expect("missing jenkins url")
    )
        .send()
        .await?;
    
    msg
        .channel_id
        .say(&ctx.http, 
            format!("Jenkins request results: {:?}", result))
        .await?;

    Ok(())
    
}

use serenity::framework::standard::{macros::{command}, 
    CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::env;
use reqwest::{header};
use serde::{Deserialize};
use std::collections::HashMap;

#[derive(Deserialize)]
struct RepositoryStatus {
    needs_commit : bool,
    needs_merge : bool,
    needs_push : bool,

}

#[derive(Deserialize)]
struct OperationResult {
    result: bool,
}

#[command]
#[aliases("s")]
#[description = "Checks weblate health, shows number of pending changes"]
async fn status(ctx: &Context, msg: &Message) -> CommandResult {
    fn beuatify_boolean(value :bool) -> &'static str {
        return match value {
            false =>  "Nope ✅",
            true => "Yeah 👀"
        }
        
    }
    
    let url = get_weblate_url();

    let client = get_weblate_client();

    let res = client.get(url).send().await?.json::<RepositoryStatus>().await?;

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Weblate status");
            e.description("Below are statuses of ");
            e.fields(vec![
                ("Commits pending", beuatify_boolean(res.needs_commit), true),
                ("Needs merge", beuatify_boolean(res.needs_merge), true),
                ("Push pending", beuatify_boolean(res.needs_push), true),
            ]);

            e
        });

        m
    }).await?;

    Ok(())
    
}




#[command]
#[aliases("u")]
async fn update(ctx: &Context, msg: &Message) -> CommandResult {
    fn get_params(operation: &str) -> HashMap<&str, &str> {
        let mut map = HashMap::new();
        map.insert("operation", operation);
        map
    }
    fn format_result(operation: &str, result: bool) -> String {
        format!("Operation {} results: {}", operation, result)
    }

    async fn run_command(operation: &str,client: &reqwest::Client, m: &Message, ctx: &Context) -> CommandResult {
        let res = client.post(get_weblate_url())
        .json(&get_params(operation))
        .send()
        .await?
        .json::<OperationResult>()
        .await?;
    
    m
        .channel_id
        .say(&ctx.http,format_result(operation, res.result))
        .await?;
        Ok(())
    }
    let client = get_weblate_client();

    run_command("commit", &client, msg, ctx).await?; 
    run_command("push", &client, msg, ctx).await?;    

    Ok(())
}


fn get_weblate_client() -> reqwest::Client {
    let mut headers = header::HeaderMap::new();
    let mut token = header::HeaderValue::from_str(&format!("Token {}",
    env::var("WEBLATE_TOKEN").expect("missing weblate token"))).expect("creating token failed");
    token.set_sensitive(true);
    headers.insert("Authorization", token);
    let client = reqwest::Client::builder()
    .default_headers(headers)
//    .proxy(reqwest::Proxy::http("http://127.0.0.1:8888").unwrap())
    .build();
    client.expect("creating client failed")
}

fn get_weblate_url() -> String {
    format!("{}/api/projects/{}/repository/", 
    env::var("WEBLATE_HOST").expect("missing weblate url"), 
    env::var("WEBLATE_PROJECT").expect("missing project name from env") )
}
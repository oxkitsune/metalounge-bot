use std::env;

use serenity::model::channel::ReactionType;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

// contains all project link channel ids
const PROJECT_CHANNELS: &[u64] = &[915434054634049666, 915441939963322410];

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // ignore bots
        if msg.author.bot {
            return;
        }

        if !PROJECT_CHANNELS.contains(&msg.channel_id.0) {
            return;
        }

        if !msg.content.starts_with("Name of project:") {
            match msg
                .reply_mention(&ctx.http, "Please follow the format posted by Projecki.")
                .await
            {
                Ok(sent_msg) => {
                    if let Err(err) = msg.delete(&ctx.http).await {
                        println!("Failed to delete original message: {:?}", err);
                    }

                    // delete message after 5 seconds
                    tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
                    if let Err(err) = sent_msg.delete(&ctx.http).await {
                        println!("Failed to delete notice message: {:?}", err)
                    }
                }
                Err(err) => {
                    println!("Failed to post message: {:?}!", err)
                }
            }

            return;
        }

        // add thumbs up emojis
        if let Err(error) = msg
            .react(&ctx.http, ReactionType::Unicode("👍".to_string()))
            .await
        {
            println!("Failed to add reaction: thumbs_up: {:?}!", error)
        }

        if let Err(error) = msg
            .react(&ctx.http, ReactionType::Unicode("👎".to_string()))
            .await
        {
            println!("Failed to add reaction: thumbs_up: {:?}!", error)
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // init bot
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

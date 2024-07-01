use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::guild::Member;
use serenity::model::id::GuildId;
use serenity::model::prelude::ChannelId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, _ready: Ready) {
        println!("connected");
    }

    async fn guild_member_addition(&self, ctx: Context, guild_id: GuildId, new_member: Member) {
        let welcome_message = format!("Welcome to the server, {}!", new_member.user.name);
        let CHANNEL_ID = 'sampleXXXxxx'; //can't push the channel id to github, so i just put a placeholder here
        // this is basically and if/else statement in one "function" (i don't know if they are methods or functions in this language), if we get an error, we print the error, if we don't get an error it sends the welcome message variable
        if let Err(why) = ChannelId(CHANNEL_ID).say(&ctx.http, welcome_message).await {
            println!("Error sending message: {:?}", why);
        }
    }
}

#[tokio::main]
async fn main() {
    let token = "XXXXXXXXXXXXXXXXXXXXXxxsample";  //can't push the token to github, so i just put a placeholder here

    let mut client = Client::builder(&token)  //event handler 
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}



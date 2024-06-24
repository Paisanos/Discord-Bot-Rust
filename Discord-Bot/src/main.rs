use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, guild::Member},
    prelude::*,
};

struct Handler;

impl EventHandler for Handler{
    async fn message(&self, ctx: Context, msg: Message){
        if msg.content == "!ping"{
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
    async fn guild_member_addition(&self, ctx: Context, new_member: Member){
        let channel_id = ChannelId();  //<=The Channel id goes inside that function
        let welcome_message = format!("Welcome to the server, {}! Click this link to add yourself to PsychNaw: https://people.psychnaw.com",
        new_member.user.mention());
    }
}



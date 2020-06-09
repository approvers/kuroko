use serenity::client::Client;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::prelude::{Context, EventHandler};
use std::env;
use std::sync::Arc;

mod messages;
mod commands {
    pub mod desuwa;
    pub mod judgement;
    pub mod time;
}

fn main() {
    let token = env::var("KUROKO_DISCORD_TOKEN").expect(messages::NO_DISCORD_TOKEN);

    let mut client = Client::new(token, KurokoEventHandler).expect(messages::DISCORD_LAUNCH_FAILED);
    client.start().expect(messages::DISCORD_LAUNCH_FAILED);
}

struct KurokoEventHandler;

impl EventHandler for KurokoEventHandler {
    fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        println!("{}", messages::READY);
    }

    fn message(&self, ctx: Context, message: Message) {
        if message.author.bot {
            return;
        }

        let content = message.content.trim();
        let splitted = content.split(" ").collect::<Vec<&str>>();
        let first_element = splitted[0];

        use commands::*;
        let text = match first_element {
            judgement::JUDGEMENT_COMMAND => judgement::judgement(&splitted),

            time::TIME_COMMAND => time::time(&content, &splitted),

            desuwa::DESUWA_COMMAND => desuwa::desuwa(&content, &splitted),

            _ => None,
        };

        if let Some(send_text) = text {
            send_message_checked(&send_text, message.channel_id, ctx.http);
        }
    }
}

const DISCORD_MESSAGE_MAX_LENGTH: usize = 2000;
fn send_message_checked(msg: &str, channel_id: ChannelId, http: Arc<Http>) {
    let text = if msg.len() >= DISCORD_MESSAGE_MAX_LENGTH {
        messages::TOO_LONG
    } else {
        msg
    };

    let result = channel_id.say(&http, text);
    if let Err(e) = result {
        println!("{}: {}", messages::MESSAGE_SEND_FAIL, e);
    }
}

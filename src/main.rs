use serenity::client::Client;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::prelude::{Context, EventHandler};
use std::env;
use std::sync::Arc;

mod messages;
use messages::CheckedMessage;

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

        if text.is_some() {
            send_message_checked(text.unwrap(), message.channel_id, ctx.http)
        }
    }
}

fn send_message_checked(msg: String, channel_id: ChannelId, http: Arc<Http>) {
    let checked_message = CheckedMessage::new(msg);

    let send_result = match checked_message {
        Ok(m) => channel_id.say(&http, m.content()),
        Err(e) => channel_id.say(&http, e),
    };

    if let Err(e) = send_result {
        println!("{}: {}", messages::MESSAGE_SEND_FAIL, e);
    }
}

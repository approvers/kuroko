use serenity::client::Client;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::prelude::{Context, EventHandler};
use std::env;
use std::sync::Arc;

mod messages;

fn main() {
    let token = env::var("KUROKO_DISCORD_TOKEN").expect(messages::NO_DISCORD_TOKEN);

    let mut client = Client::new(token, KurokoEventHandler).unwrap();
    client.start().unwrap();
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

        match first_element {
            "!judgement" => {
                if splitted.len() != 1 {
                    return;
                }

                send_message_checked(messages::JUDGEMENT, message.channel_id, ctx.http);
            }

            "!time" => {
                if splitted.len() <= 1 {
                    send_message_checked(
                        messages::NOT_ENOUGH_ARGUMENTS,
                        message.channel_id,
                        ctx.http,
                    );
                    return;
                }

                let daggered = if let Some(s) = splitted.get(2) {
                    *s == "--dagger"
                } else {
                    false
                };

                let text = messages::ITS_THE_TIME(splitted.get(1).unwrap(), daggered);
                send_message_checked(&text, message.channel_id, ctx.http);
            }

            "!desuwa" => {
                if splitted.len() <= 1 {
                    send_message_checked(
                        messages::NOT_ENOUGH_ARGUMENTS,
                        message.channel_id,
                        ctx.http,
                    );
                    return;
                }

                let text = messages::DESUWA(splitted.get(1).unwrap());
                send_message_checked(&text, message.channel_id, ctx.http);
            }

            _ => {}
        }
    }
}

fn send_message_checked(msg: &str, channel_id: ChannelId, http: Arc<Http>) {
    let result = channel_id.say(&http, msg);
    if let Err(e) = result {
        println!("{}: {}", messages::MESSAGE_SEND_FAIL, e);
    }
}
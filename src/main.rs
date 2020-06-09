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

    let mut client = Client::new(token, KurokoEventHandler).expect(messages::DISCORD_LAUNCH_FAILED);
    client.start().expect(messages::DISCORD_LAUNCH_FAILED);
}

const JUDGEMENT_COMMAND: &str = "!judgement";
const TIME_COMMAND: &str = "!time";
const APPEND_DAGGER_OPTION: &str = "--dagger";
const APPEND_DAGGER_OPTION_SHORT: &str = "-d";
const DESUWA_COMMAND: &str = "!desuwa";
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

        let text = match first_element {
            JUDGEMENT_COMMAND => {
                if splitted.len() != 1 {
                    None
                } else {
                    Some(messages::JUDGEMENT.to_string())
                }
            }

            TIME_COMMAND => {
                if splitted.len() <= 1 {
                    Some(messages::NOT_ENOUGH_ARGUMENTS.to_string())
                } else {
                    let daggered = splitted.get(2).map_or(false, |s| {
                        *s == APPEND_DAGGER_OPTION || *s == APPEND_DAGGER_OPTION_SHORT
                    });

                    let arg = if daggered {
                        splitted.get(1).unwrap().to_string()
                    } else {
                        content.chars().skip(TIME_COMMAND.len()).collect::<String>()
                    };

                    Some(messages::ITS_THE_TIME(arg.trim(), daggered))
                }
            }

            DESUWA_COMMAND => {
                if splitted.len() <= 1 {
                    Some(messages::NOT_ENOUGH_ARGUMENTS.to_string())
                } else {
                    let arg = content.chars().skip("!desuwa".len()).collect::<String>();
                    Some(messages::DESUWA(arg.trim()))
                }
            }

            _ => None,
        };

        if let Some(send_text) = text {
            send_message_checked(&send_text, message.channel_id, ctx.http);
        }
    }
}

fn send_message_checked(msg: &str, channel_id: ChannelId, http: Arc<Http>) {
    let text = if msg.len() >= 2000 {
        messages::TOO_LONG
    } else {
        msg
    };

    let result = channel_id.say(&http, text);
    if let Err(e) = result {
        println!("{}: {}", messages::MESSAGE_SEND_FAIL, e);
    }
}

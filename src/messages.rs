macro_rules! kuroko {
    ($e:expr) => {concat!("おﾈｪｻﾏｰｰｰｯｯｯｯｯ!!!!!", $e, "ｯｯｯｯｯｯｯｯ!!!!!!")}
}

pub const NO_DISCORD_TOKEN: &str = kuroko!("トークンがございませんの");
pub const DISCORD_LAUNCH_FAILED: &str = kuroko!("Discordに接続できませんの");
pub const READY: &str = kuroko!("準備完了ですの");
pub const JUDGEMENT: &str = "ジャッジメントですのっ！";
pub const MESSAGE_SEND_FAIL: &str = kuroko!("発言できませんの");
pub const NOT_ENOUGH_ARGUMENTS: &str = kuroko!("引数が足りませんの");
pub const TOO_LONG: &str = kuroko!("メッセージが長すぎて送信できませんの");

#[allow(non_snake_case)]
pub fn ITS_THE_TIME(text: &str, daggered: bool) -> String {
    if daggered {
        format!("†{}†のお時間ですわよ!", text)
    } else {
        format!("{}のお時間ですわよ!", text)
    }
}

#[allow(non_snake_case)]
pub fn DESUWA(text: &str) -> String {
    format!("{}ですわ！", text)
}

const DISCORD_MESSAGE_MAX_LENGTH: usize = 2000;
pub struct MessageTooLongError(String);
impl std::fmt::Display for MessageTooLongError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

pub struct CheckedMessage {
    content: String,
}

impl CheckedMessage {
    pub fn new(content: String) -> Result<Self, MessageTooLongError> {
        if content.len() >= DISCORD_MESSAGE_MAX_LENGTH {
            Err(MessageTooLongError(TOO_LONG.into()))
        } else {
            Ok(Self { content })
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

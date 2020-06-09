pub const NO_DISCORD_TOKEN: &str = "おﾈｪｻﾏｰｰｰｯｯｯｯｯ!!!!! トークンがございませんのｯｯｯｯｯｯｯ!!!!!";
pub const READY: &str = "おﾈｪｻﾏｰｰｰｯｯｯｯｯ!!!!! 準備完了ですのｯｯｯｯｯｯｯｯ!!!!!!";
pub const JUDGEMENT: &str = "ジャッジメントですのっ！";
pub const MESSAGE_SEND_FAIL: &str = "おﾈｪｻﾏｰｰｰｯｯｯｯｯ!!!!! 発言出来ませんのｯｯｯｯｯｯｯ!!!!!";
pub const NOT_ENOUGH_ARGUMENTS: &str = "おﾈｪｻﾏｰｰｰｯｯｯｯｯ!!!!! 引数が足りませんのｯｯｯｯｯｯ!!!!!!";

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

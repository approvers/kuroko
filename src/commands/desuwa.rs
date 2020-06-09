use crate::messages;

pub const DESUWA_COMMAND: &str = "!desuwa";

pub fn desuwa(content: &str, splitted_text: &Vec<&str>) -> Option<String> {
    if splitted_text.len() <= 1 {
        return Some(messages::NOT_ENOUGH_ARGUMENTS.to_string());
    }

    let arg = content
        .chars()
        .skip(DESUWA_COMMAND.len())
        .collect::<String>();

    return Some(messages::DESUWA(arg.trim()));
}

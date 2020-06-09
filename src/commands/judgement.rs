use crate::messages;

pub const JUDGEMENT_COMMAND: &str = "!judgement";

pub fn judgement(splitted_message: &Vec<&str>) -> Option<String> {
    if splitted_message.len() != 1 {
        None
    } else {
        Some(messages::JUDGEMENT.to_string())
    }
}

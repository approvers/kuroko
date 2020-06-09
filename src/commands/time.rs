use crate::messages;

const APPEND_DAGGER_OPTION: &str = "--dagger";
const APPEND_DAGGER_OPTION_SHORT: &str = "-d";

pub const TIME_COMMAND: &str = "!time";

pub fn time(content: &str, splitted_text: &Vec<&str>) -> Option<String> {
    if splitted_text.len() <= 1 {
        return Some(messages::NOT_ENOUGH_ARGUMENTS.to_string());
    }

    let daggered = splitted_text.get(2).map_or(false, |s| {
        *s == APPEND_DAGGER_OPTION || *s == APPEND_DAGGER_OPTION_SHORT
    });

    let arg = if daggered {
        splitted_text.get(1).unwrap().to_string()
    } else {
        content.chars().skip(TIME_COMMAND.len()).collect::<String>()
    };

    Some(messages::ITS_THE_TIME(arg.trim(), daggered))
}

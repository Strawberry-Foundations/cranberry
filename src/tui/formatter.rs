use stblib::colors::*;
use std::collections::HashMap;

#[derive(Clone, Copy, Default)]
pub enum MessageFormats {
    #[default]
    Default,
}

pub type MessageFormat = HashMap<&'static str, (String, String, String)>;

pub struct MessageFormatter {
    pub format: MessageFormats,
    pub format_str: String,
    pub formats: MessageFormat,
    pub current_format: (String, String, String),
}

impl MessageFormatter {
    pub fn load_formats() -> MessageFormat {
        let mut formats = HashMap::new();

        formats.insert("default", (
            format!("{GRAY}[[%<time>%]]{C_RESET}{WHITE} [%<message>%]"),
            format!("{GRAY}[[%<time>%]]{C_RESET} [%<role_color>%][%<username>%][%<badge>%]:{C_RESET} [%<message>%]{C_RESET}"),
            format!("{GRAY}[[%<time>%]]{C_RESET} [%<role_color>%][%<nickname>%] (@[%<username>%])[%<badge>%]:{C_RESET} [%<message>%]{C_RESET}")
        ));

        formats
    }

    pub fn new() -> Self {
        let fmt = "default";

        let format = match fmt {
            "default" => MessageFormats::Default,
            &_ => MessageFormats::Default,
        };

        let format_str = match format {
            MessageFormats::Default => String::from("default"),
        };

        let formats = Self::load_formats();

        let current_format = formats
            .get_key_value(format_str.as_str())
            .unwrap()
            .1
            .to_owned();

        Self {
            format,
            format_str,
            formats,
            current_format,
        }
    }

    pub fn user(
        &self,
        username: String,
        nickname: String,
        role_color: String,
        badge: String,
        message: String,
    ) -> String {
        match nickname {
            _ if username == nickname => self
                .current_format
                .1
                .replace("[%<time>%]", &stblib::utilities::current_time("%H:%M"))
                .replace("[%<role_color>%]", &role_color)
                .replace("[%<username>%]", &username)
                .replace("[%<role_color>%]", &nickname)
                .replace("[%<badge>%]", &badge)
                .replace("[%<message>%]", &message),
            _ => self
                .current_format
                .2
                .replace("[%<time>%]", &stblib::utilities::current_time("%H:%M"))
                .replace("[%<role_color>%]", &role_color)
                .replace("[%<username>%]", &username)
                .replace("[%<nickname>%]", &nickname)
                .replace("[%<role_color>%]", &nickname)
                .replace("[%<badge>%]", &badge)
                .replace("[%<message>%]", &message),
        }
    }

    pub fn system(&self, message: String) -> String {
        self.current_format
            .0
            .replace("[%<time>%]", &stblib::utilities::current_time("%H:%M"))
            .replace("[%<message>%]", &message)
    }
}

pub fn badge_handler(badge: String) -> String {
    if !badge.is_empty() {
        format!(" [{}]", badge)
    } else {
        String::new()
    }
}

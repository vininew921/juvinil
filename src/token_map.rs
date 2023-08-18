pub const T_KEYWORD_INIT: &str = "<keyword init>";
pub const T_OP: &str = "<op %s>";
pub const T_INT: &str = "<int %s>";
pub const T_STRING: &str = "<string %s>";
pub const T_ID: &str = "<id %s>";

pub const TOKEN_MAP: [RegexToken<&'static str>; 4] = [
    RegexToken::new(r#"[+\-*%/=^&|!]"#, T_OP, false),
    RegexToken::new(r#"^\d+$"#, T_INT, false),
    RegexToken::new(r#"^"[^"]*"$"#, T_STRING, true),
    RegexToken::new(r#"^[A-Za-z]+"#, T_ID, false),
];

pub struct RegexToken<T: AsRef<str>> {
    pub regex_template: T,
    pub token_template: T,
    pub remove_quotes: bool,
}

impl RegexToken<&'static str> {
    pub const fn new(
        regex_template: &'static str,
        token_template: &'static str,
        remove_quotes: bool,
    ) -> Self {
        Self {
            regex_template,
            token_template,
            remove_quotes,
        }
    }

    pub fn format_token(&self, token: &str) -> String {
        if self.remove_quotes {
            return self.token_template.replace("%s", token).replace(r#"""#, "");
        }

        self.token_template.replace("%s", token)
    }
}

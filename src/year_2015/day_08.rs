use once_cell::sync::Lazy;
use regex::Regex;

pub fn reformat_string(before: &str) -> String {
    static DOUBLE_QUOTES_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#""(?P<s>\S+)?""#).unwrap());
    static HEX_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\x[0-9a-fA-F]{2}").unwrap());
    static ESCAPED_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\(?P<s>\S)").unwrap());

    let s = DOUBLE_QUOTES_REGEX.replace(&before, "$s");
    let s = HEX_REGEX.replace_all(&s, "'");
    ESCAPED_REGEX.replace_all(&s, "$s").to_string()
}

pub fn escape_string(before: &str) -> String {
    static ESCAPE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(?P<s>[\\"])"#).unwrap());
    static DOUBLE_QUOTES_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?P<s>\S+)").unwrap());

    let s = ESCAPE_REGEX.replace_all(&before, r"\$s");
    DOUBLE_QUOTES_REGEX.replace(&s, r#""$s""#).to_string()
}

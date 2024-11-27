use syntect::{easy::HighlightLines, highlighting::{Style, ThemeSet}, parsing::SyntaxSet, util::{as_24_bit_terminal_escaped, LinesWithEndings}};
use crate::result_trait::RfocResultExtended;




pub fn enable_syntax(text: &str, extension: &str, theme: &str) -> String
{
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    match ps.find_syntax_by_extension(extension){
        Some(syntax) => {
            let theme = &ts.themes[theme];

            let mut h = HighlightLines::new(syntax, theme);
            let mut ret: Vec<String> = vec![];

            for line in LinesWithEndings::from(text){
                let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).rfoc_unwrap();
                let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
                ret.push(escaped);
            }
            let ret = ret.join("");
            return ret;
        }
        None => {
            return text.to_string();
        }
    }
}

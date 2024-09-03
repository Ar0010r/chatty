use regex::Regex;

pub mod gmail;
pub mod shared;

fn retrieve_from_str(input: &str) -> Option<String> {
    let regex = Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b").unwrap();
    regex.captures(input).map(|capture| capture[0].to_string())
}

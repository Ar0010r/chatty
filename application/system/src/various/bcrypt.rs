use regex::Regex;

static COST: u32 = 12;
static REGEX: &str = r"^\$2[ayb]\$.{56}$";

pub fn hash(value: &str) -> String {
    bcrypt::hash(value, COST).unwrap()
}

pub fn hash_once(value: &str) -> String {
    let bcrypt_regex = Regex::new(REGEX).unwrap();

    match bcrypt_regex.is_match(value) {
        true => value.to_string(),
        false => hash(value),
    }
}

pub fn verify(value: &str, hash: &str) -> bool {
    bcrypt::verify(value, hash).unwrap_or_default()
}

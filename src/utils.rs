//normalizeing token by lowecasing and removing non-alphanumeric characters
// A -> a

pub fn normalize_token(token: &str) -> String {
    token.to_lowercase()
    .chars()
    .filte(|c|c.is_alphanumeric())
    .collect()
}


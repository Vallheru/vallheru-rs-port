pub fn hash_password(pass: &str) -> String {
    bcrypt::hash(pass.as_bytes(), bcrypt::DEFAULT_COST).unwrap_or(String::from("invalid password"))
}

pub fn is_valid_password(pass: &str, hash: &str) -> bool {
    bcrypt::verify(pass.as_bytes(), hash).unwrap_or(false)
}

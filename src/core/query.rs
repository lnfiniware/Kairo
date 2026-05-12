pub fn translate_query(query: &str) -> String {
    let trimmed = query.trim();
    
    // Very basic natural query translation
    // "from users" -> "SELECT * FROM users"
    if trimmed.to_lowercase().starts_with("from ") {
        format!("SELECT * {}", trimmed)
    } else {
        trimmed.to_string()
    }
}

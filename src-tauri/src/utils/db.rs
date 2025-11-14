pub fn extract_field_from_constraint(c: &str) -> Option<String> {
    let parts: Vec<&str> = c.split('_').collect();
    if parts.len() >= 3 {
        Some(parts[parts.len() - 2].to_string())
    } else {
        None
    }
}

pub fn pg_violation(code: &str) -> Option<&'static str> {
    match code {
        "23505" => Some("Value already exists"), // unique violation
        "23502" => Some("Value must not be empty"), // not null violation
        "23514" => Some("Check constraint violation"), // check violation
        _ => None,
    }
}

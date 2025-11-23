/// Extracts the field name from a PostgreSQL constraint name.
///
/// This function assumes constraint names follow the pattern:
/// `{table_name}_{field_name}_{constraint_type}`
///
/// Examples:
/// - `profiles_username_key` → `Some("username")`
/// - `jobs_salary_gross_check` → `Some("gross")`
/// - `invalid` → `None`
///
/// # Arguments
/// * `constraint_name` - The full constraint name from PostgreSQL
///
/// # Returns
/// * `Some(String)` - The extracted field name if the pattern matches
/// * `None` - If the constraint name doesn't follow the expected pattern
pub fn extract_field_from_constraint(constraint_name: &str) -> Option<String> {
    let parts: Vec<&str> = constraint_name.split('_').collect();

    if parts.len() >= 3 {
        Some(parts[parts.len() - 2].to_string())
    } else {
        None
    }
}

/// Maps PostgreSQL error codes to human-readable messages.
///
/// Common PostgreSQL constraint violation error codes:
/// - `23505`: UNIQUE violation - attempting to insert/update a duplicate value
/// - `23502`: NOT NULL violation - attempting to insert/update a NULL value
/// - `23514`: CHECK constraint violation - value doesn't satisfy a CHECK constraint
/// - `23503`: FOREIGN KEY violation - referencing non-existent row
///
/// # Arguments
/// * `code` - The PostgreSQL error code (SQLSTATE)
///
/// # Returns
/// * `Some(&str)` - A human-readable error message for known codes
/// * `None` - For unknown error codes
pub fn pg_violation(code: &str) -> Option<&'static str> {
    match code {
        "23505" => Some("Value already exists"),
        "23502" => Some("Value must not be empty"),
        "23514" => Some("Check constraint violation"),
        "23503" => Some("Referenced record does not exist"),
        _ => None,
    }
}

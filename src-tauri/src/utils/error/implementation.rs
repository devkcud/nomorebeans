use crate::utils::{
    db,
    error::mapping::{ErrorCode, ErrorResponse},
};

impl ErrorCode {
    pub fn raise(self, field: impl Into<String>, message: impl Into<String>) -> ErrorResponse {
        ErrorResponse::new(self, Some(field.into()), message)
    }

    pub fn msg_only(self, message: impl Into<String>) -> ErrorResponse {
        ErrorResponse::new(self, None, message)
    }
}

impl ErrorResponse {
    pub fn new(code: ErrorCode, field: Option<String>, message: impl Into<String>) -> Self {
        if let ErrorCode::PlaceholderError = code {
            panic!("PlaceholderError must not be used in real errors");
        }

        Self {
            code_name: code,
            code: code.into(),
            field,
            message: message.into(),
        }
    }

    pub fn known_internal() -> Self {
        ErrorCode::KnownInternalError.msg_only("Expected error")
    }

    pub fn unhandled() -> Self {
        ErrorCode::UnhandledError.msg_only("Unhandled error")
    }

    pub fn object_not_found(field: impl Into<String>, message: impl Into<String>) -> Self {
        ErrorCode::SearchObjectNotFoundError.raise(field, message)
    }

    pub fn validation_error(field: impl Into<String>, message: impl Into<String>) -> Self {
        ErrorCode::UserInputValidationError.raise(field, message)
    }

    pub fn with_field(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }
}

impl From<sqlx::Error> for ErrorResponse {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::Database(db_err) => {
                let code = match db_err.code() {
                    Some(c) => c,
                    None => return ErrorCode::DatabaseError.msg_only("Database error"),
                };

                let msg = match db::pg_violation(code.as_ref()) {
                    Some(m) => m,
                    None => return ErrorCode::DatabaseError.msg_only("Database error"),
                };

                let field = db_err
                    .constraint()
                    .and_then(|c| db::extract_field_from_constraint(c));

                match field {
                    Some(f) => ErrorCode::UserInputValidationError.raise(f, msg),
                    None => ErrorCode::UserInputValidationError.msg_only(msg),
                }
            }
            sqlx::Error::RowNotFound => ErrorCode::ResourceError.msg_only("Resource not found"),
            _ => ErrorResponse::unhandled(),
        }
    }
}

impl From<std::io::Error> for ErrorResponse {
    fn from(err: std::io::Error) -> Self {
        use std::io::ErrorKind::*;

        let (msg, error_code) = match err.kind() {
            NotFound => ("File or resource not found", ErrorCode::FileRWError),
            PermissionDenied => ("Permission denied", ErrorCode::InsufficientPrivilegesError),
            ConnectionRefused | ConnectionReset | ConnectionAborted => {
                ("Connection error occurred", ErrorCode::NetworkError)
            }
            TimedOut => ("I/O operation timed out", ErrorCode::IOError),
            UnexpectedEof => ("Unexpected end of file", ErrorCode::FileRWError),
            _ => ("I/O error occurred", ErrorCode::IOError),
        };

        error_code.msg_only(msg)
    }
}

impl From<validator::ValidationErrors> for ErrorResponse {
    fn from(err: validator::ValidationErrors) -> Self {
        let field_detail = err.field_errors().iter().next().map(|(field, errs)| {
            let msg = errs
                .first()
                .and_then(|e| e.message.as_ref())
                .map(|m| m.to_string())
                .unwrap_or_else(|| "Invalid value".to_string());

            (field.to_string(), msg)
        });

        if let Some((field, msg)) = field_detail {
            ErrorCode::UserInputValidationError.raise(field, msg)
        } else {
            ErrorCode::UserInputValidationError.msg_only("Validation error occurred")
        }
    }
}

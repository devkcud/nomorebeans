use crate::utils::{
    db,
    error::mapping::{ErrorCode, ErrorResponse},
};

impl ErrorResponse {
    pub fn new(code: ErrorCode, field: Option<String>, message: impl Into<String>) -> Self {
        Self {
            code_name: code,
            code: code.into(),
            field,
            message: message.into(),
        }
    }

    pub fn expected() -> Self {
        Self::new(ErrorCode::ExpectedError, None, "Expected error")
    }

    pub fn unhandled() -> Self {
        Self::new(ErrorCode::UnhandledError, None, "Unhandled error")
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
                if let Some(code) = db_err.code() {
                    if let Some(msg) = db::pg_violation(code.as_ref()) {
                        let field = db_err
                            .constraint()
                            .and_then(|c| db::extract_field_from_constraint(c));

                        return ErrorResponse::new(ErrorCode::UserInputValidationError, field, msg);
                    }
                }
                ErrorResponse::new(ErrorCode::DatabaseError, None, "Database error")
            }
            sqlx::Error::RowNotFound => {
                ErrorResponse::new(ErrorCode::ResourceError, None, "Resource not found")
            }
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

        ErrorResponse::new(error_code, None, msg)
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
            ErrorResponse::new(ErrorCode::UserInputValidationError, Some(field), msg)
        } else {
            ErrorResponse::new(ErrorCode::ExpectedError, None, "Validation error occurred")
        }
    }
}

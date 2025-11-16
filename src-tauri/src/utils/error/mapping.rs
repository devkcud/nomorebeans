use serde::Serialize;

/* Number format: ABCC

    A - Error category
    B - Severity
    C - Specific error code (1 to 99)

    General rules:
    - Category and Severity alike must follow a scale from 1-9, where 1 is closest to the client and 9 is closest to the server. Or in rare cases, 0 for informational purposes.
    - Validation errors should always have severity 1 (Client).
    - IO errors are rarely tied to client actions, so they should have severity 2 or higher.
*/

#[allow(non_upper_case_globals)]
mod ec_cat {
    pub const Generic: u32 = 0;
    pub const Validation: u32 = 1;
    pub const IO: u32 = 3;
    pub const Permission: u32 = 4;
    pub const Service: u32 = 5;
    pub const Undefined: u32 = 9;
}

#[allow(non_upper_case_globals)]
mod ec_sev {
    pub const Info: u32 = 0;
    pub const Client: u32 = 1;
    pub const Server: u32 = 2;
    pub const Critical: u32 = 9;
}

macro_rules! error_codes {
    (
        $(
            $variant:ident = $cat:ident $sev:ident $cc:literal ;
        )+
    ) => {
        #[derive(Debug, Clone, Copy, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        pub enum ErrorCode {
            $($variant,)+
        }

        impl From<ErrorCode> for u32 {
            fn from(code: ErrorCode) -> Self {
                match code {
                    $(
                        ErrorCode::$variant => {
                            let a = crate::utils::error::mapping::ec_cat::$cat;
                            let b = crate::utils::error::mapping::ec_sev::$sev;

                            let cc: u32 = $cc;
                            debug_assert!(cc > 0 && cc < 100);

                            a * 1000 + b * 100 + cc
                        }
                    )+
                }
            }
        }
    }
}

error_codes! {
    UserInputValidationError  = Validation Client 01;
    SearchObjectNotFoundError = Validation Client 02;

    ExpectedError = Service Server 01; // Used for known, server, unhandled errors. Example: database errors, IO errors, etc.
    DatabaseError = Service Server 02;

    ResourceError = Service Client 01; // Not tied to user input. Example: resource not found, etc.

    IOError     = IO Server 01;
    FileRWError = IO Server 02;

    NetworkError = IO Critical 01;

    InsufficientPrivilegesError = Permission Client 01;

    MisconfiguredPrivilegesError = Permission Server 01;

    UnhandledError = Undefined Server 01; // Used for generic unhandled errors. Example: default cases, panics, overflows, etc. Should be rarely used.

    PlaceholderError = Generic Info 00; // Must not be used for real errors, only type-checking, default errors and serves as a reminder. Expected to fail if ever used in development, expected to be silent in production.
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub code_name: ErrorCode,
    pub code: u32,

    pub field: Option<String>,
    pub message: String,
}

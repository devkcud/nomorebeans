use regex::Regex;
use std::sync::LazyLock;

pub static USERNAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-z0-9]+$").expect("Invalid USERNAME_REGEX pattern"));

pub const USERNAME_MIN_LENGTH: u64 = 3;
pub const USERNAME_MAX_LENGTH: u64 = 16;
pub const USERNAME_LENGTH_MESSAGE: &str = "Username must be between 3 and 16 characters";
pub const USERNAME_PATTERN_MESSAGE: &str =
    "Username can only contain lowercase letters and numbers";

pub const DISPLAY_NAME_MIN_LENGTH: u64 = 1;
pub const DISPLAY_NAME_MAX_LENGTH: u64 = 32;
pub const DISPLAY_NAME_LENGTH_MESSAGE: &str = "Display name must be between 1 and 32 characters";

pub const PROFILE_PICTURE_MAX_SIZE: usize = 2 * 1024 * 1024;
pub const PROFILE_PICTURE_SIZE_MESSAGE: &str = "Profile picture must be less than 2MB";

pub const COMPANY_NAME_MIN_LENGTH: u64 = 1;
pub const COMPANY_NAME_MAX_LENGTH: u64 = 64;
pub const COMPANY_NAME_LENGTH_MESSAGE: &str = "Company name must be between 1 and 64 characters";

pub const POSITION_TITLE_MIN_LENGTH: u64 = 1;
pub const POSITION_TITLE_MAX_LENGTH: u64 = 64;
pub const POSITION_TITLE_LENGTH_MESSAGE: &str =
    "Position title must be between 1 and 64 characters";

pub const DAILY_WORK_HOURS_MIN: i32 = 1;
pub const DAILY_WORK_HOURS_MAX: i32 = 24;
pub const DAILY_WORK_HOURS_RANGE_MESSAGE: &str = "Daily work hours must be between 1 and 24";

pub const WORKDAYS_PER_MONTH_MIN: i32 = 1;
pub const WORKDAYS_PER_MONTH_MAX: i32 = 30;
pub const WORKDAYS_PER_MONTH_RANGE_MESSAGE: &str = "Workdays per month must be between 1 and 30";

pub const SALARY_GROSS_NEGATIVE_MESSAGE: &str = "Salary gross must be non-negative";

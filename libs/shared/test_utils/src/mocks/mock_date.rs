use std::sync::LazyLock;

use chrono::{DateTime, Local, TimeZone};

/// Mock date used for testing
/// 2025-01-01
pub static MOCK_DATE: LazyLock<DateTime<Local>> = LazyLock::new(|| {
    return Local.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
});

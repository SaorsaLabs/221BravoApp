#![warn(non_snake_case)]
pub const VERSION:&str = "0.2.1";
pub const MAX_TOTAL_DOWNLOAD: usize = 10_000; 
pub const MAX_TRANSACTION_BATCH_SIZE: usize = 1000;
pub const HOUR_AS_NANOS: u64 = 3600_000_000_000;
pub const DAY_AS_NANOS: u64 = 86_400_000_000_000;
pub const MAX_DAYS: u64 = 30; 
pub const MAX_HOURS: u64 = 48;
pub const STATS_RETURN_LENGTH: usize = 10;
pub const MAX_LOG_LENGTH: usize = 500;

//! Result codes in the SMC calling convention.

pub const SUCCESS: u32 = 0;
pub const UNIMPLEMENTED: u32 = 1;
pub const INVALID_ARGUMENT: u32 = 2;
pub const IN_PROGRESS: u32 = 3;
pub const NO_ASYNC_OPERATION: u32 = 4;
pub const INVALID_ASYNC_OPERATION: u32 = 5;
pub const NOT_PERMITTED: u32 = 6;

pub const UNKNOWN_FUNCTION_ID: u32 = 0xFFFF_FFFF;

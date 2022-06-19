//! Early `r0` initialization code for the Kernel.
//!
//! This is inspired by `crt0` and provides the routines needed
//! to do early kernel initialization after it is started.

// WARNING: The soundness of the code in this module relies entirely
// on avoiding the use of stack memory which is not initialized at
// this point. Thus, proceed with caution when making changes below.

pub mod cache;
pub mod el;

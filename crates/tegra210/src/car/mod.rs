//! Abstractions over the Tegra X1 Clock and Reset Controller functionality.
//!
//! See Chapter 5 in the Tegra X1 Technical Reference Manual for details.
//!
//! # Description
//!
//! The Clock and Reset (CAR) block contains all the logic needed to control most of the clocks
//! and resets to the Tegra X1 device. It takes care of most clock source programming and most
//! of the clock dividers.

pub mod raw;

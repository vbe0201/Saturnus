//! Driver for the Tegra X1 Universal Asynchronous Receiver/Transmitter Controller.
//!
//! See Chapter 36 in the Tegra X1 Technical Reference Manual for details.
//!
//! # Description
//!
//! There are five UARTs available in total. The UARTs A through D, which are
//! identical, are built into Tegra X1 devices and the fifth UART is located
//! in the Audio Processing Engine.
//!
//! These UARTs support both, 16450 and 16550 compatible modes, although this
//! implementation specifically targets the 16550 mode.

pub mod raw;

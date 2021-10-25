#[path = "nintendo_nx/smc.rs"]
mod smc;

pub use smc::{generate_random_bytes, init};

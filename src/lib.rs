pub mod config;
pub mod error;

pub mod entropy;
pub mod seed;
pub mod topology;
pub mod compound;
pub mod recycler;
pub mod memory;
pub mod finalizer;
pub mod engine;

pub use config::XcryptConfig;
pub use error::XcryptError;

pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut diff = 0u8;

    for i in 0..a.len() {
        diff |= a[i] ^ b[i];
    }

    diff == 0
}
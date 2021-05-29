use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::core::chain::Chain;

pub static BLOCK_CHAIN: Lazy<Mutex<Chain>> = Lazy::new(|| {
    Mutex::new(Chain::new())
});
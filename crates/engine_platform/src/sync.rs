pub use implementation::{Mutex, MutexGuard};

#[cfg(feature = "std")]
use std::sync as implementation;
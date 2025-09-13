extern crate alloc;
extern crate proc_macro;

mod engine_manifest;

pub mod prelude {
    pub use super::engine_manifest::*;
}

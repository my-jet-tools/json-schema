mod json_type_description;
pub use json_type_description::*;

mod opt_of;
mod vec_of;

pub mod deserializer;

/// Used by the code `ApplyJsonSchema` generates, so a user crate needs no `async-trait` dependency of its own.
pub use async_trait;
pub use json_schema_macros as macros;
pub use my_json;
pub use rust_extensions;

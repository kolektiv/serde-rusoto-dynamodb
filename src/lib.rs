pub mod de;
pub mod result;
pub mod ser;

#[macro_use]
extern crate derive_new;

// -----------------------------------------------------------------------------

// Attribute Value Serialization/Deserialization Functions

// The small public interface for ser/de, which may well be augmented at a later
// stage with a higher level set of functions for assuming a top level "map"
// type for easy integration with AWS (Rusoto) APIs.

pub use de::from_attribute_value;
pub use ser::to_attribute_value;

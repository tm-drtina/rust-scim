mod schema;
mod schema_attribute;
mod schema_attribute_mutability;
mod schema_attribute_returned;
mod schema_attribute_type;
mod schema_attribute_uniqueness;

pub use schema::Schema;
pub use schema_attribute::SchemaAttribute;
pub use schema_attribute_mutability::SchemaAttributeMutability;
pub use schema_attribute_returned::SchemaAttributeReturned;
pub use schema_attribute_type::SchemaAttributeType;
pub use schema_attribute_uniqueness::SchemaAttributeUniqueness;

pub const ENDPOINT: &str = "/Schemas";

mod generate;
mod generate_deserialize;
mod generate_serialize;
mod generate_struct;

use std::collections::BTreeSet;
use std::iter::once;

use crate::protocol::{Extensions, Meta};
use crate::resource::ScimSchema;

pub struct SchemaMismatch<'a> {
    pub missing: Vec<&'a str>,
    pub extra: Vec<&'a str>,
}

pub trait ScimResponse {
    type Resource: ScimSchema;
    type Extensions: Extensions;

    fn from_parts(
        id: String,
        external_id: Option<String>,
        meta: Meta,
        resource: Self::Resource,
        extensions: Self::Extensions,
    ) -> Self;
    fn into_parts(
        self,
    ) -> (
        String,
        Option<String>,
        Meta,
        Self::Resource,
        Self::Extensions,
    );

    fn schemas(&self) -> &[String];
    fn id(&self) -> &str;
    fn external_id(&self) -> Option<&str>;
    fn meta(&self) -> &Meta;

    fn defined_schemas() -> impl Iterator<Item = &'static str> {
        once(<Self::Resource as ScimSchema>::SCHEMA)
            .chain(<Self::Extensions as Extensions>::SCHEMA.iter().copied())
    }

    fn validate_schema<'a>(&'a self) -> Result<(), SchemaMismatch<'a>> {
        let actual_schemas: BTreeSet<&str> = self.schemas().iter().map(String::as_str).collect();
        let expected_schemas: BTreeSet<&str> = Self::defined_schemas().collect();
        if actual_schemas == expected_schemas {
            Ok(())
        } else {
            let missing = expected_schemas
                .difference(&actual_schemas)
                .copied()
                .collect();
            let extra = actual_schemas
                .difference(&expected_schemas)
                .copied()
                .collect();
            Err(SchemaMismatch { missing, extra })
        }
    }
}

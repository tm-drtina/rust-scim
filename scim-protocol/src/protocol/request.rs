mod generate;
mod generate_deserialize;
mod generate_serialize;
mod generate_struct;

use crate::protocol::Extensions;
use crate::resource::ScimSchema;

pub trait ScimRequest {
    type Resource: ScimSchema;
    type Extensions: Extensions;

    fn from_parts(
        external_id: Option<String>,
        resource: Self::Resource,
        extensions: Self::Extensions,
    ) -> Self;
    fn into_parts(self) -> (Option<String>, Self::Resource, Self::Extensions);

    fn external_id(&self) -> Option<&str>;
    fn resource(&self) -> &Self::Resource;
    fn extensions(&self) -> &Self::Extensions;
}

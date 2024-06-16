mod group;
mod member;

pub use group::Group;
pub use member::Member;

use crate::resource::ScimSchema;

impl ScimSchema for Group {
    const SCHEMA: &'static str = "urn:ietf:params:scim:schemas:core:2.0:Group";
}

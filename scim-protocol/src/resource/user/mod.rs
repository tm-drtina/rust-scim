mod address;
mod email;
mod entitlement;
mod group_membership;
mod instant_messaging;
mod name;
mod phone_number;
mod photo;
mod role;
mod user;
mod x509_certificate;

pub use address::Address;
pub use email::Email;
pub use entitlement::Entitlement;
pub use group_membership::GroupMembership;
pub use instant_messaging::InstantMessaging;
pub use name::Name;
pub use phone_number::PhoneNumber;
pub use photo::Photo;
pub use role::Role;
pub use user::User;
pub use x509_certificate::X509Certificate;

use crate::resource::ScimSchema;

impl ScimSchema for User {
    const SCHEMA: &'static str = "urn:ietf:params:scim:schemas:core:2.0:User";
}

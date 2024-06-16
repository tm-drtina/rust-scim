use serde::{Deserialize, Serialize};

/// The user's manager.
/// A complex type that optionally allows service providers to represent organizational hierarchy by referencing the "id" attribute of another User.
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Manager {
    /// The "id" of the SCIM resource representing the user's manager.
    /// RECOMMENDED.
    pub value: Option<String>,

    /// The URI of the SCIM resource representing the User's manager.
    /// RECOMMENDED.
    #[serde(rename = "$ref")]
    pub reference: Option<String>,

    /// The displayName of the user's manager.
    /// This attribute is OPTIONAL, and mutability is "readOnly".
    pub display_name: Option<String>,
}

use super::Manager;

/// The following SCIM extension defines attributes commonly used in representing users that belong to, or act on behalf of, a business or enterprise.
/// The enterprise User extension is identified using the following schema URI: "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User".
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnterpriseUser {
    /// A string identifier, typically numeric or alphanumeric, assigned to a person, typically based on order of hire or association with an organization.
    pub employee_number: Option<String>,

    /// Identifies the name of a cost center.
    pub cost_center: Option<String>,

    /// Identifies the name of an organization.
    pub organization: Option<String>,

    /// Identifies the name of a division.
    pub division: Option<String>,

    /// Identifies the name of a department.
    pub department: Option<String>,

    /// The user's manager.
    /// A complex type that optionally allows service providers to represent organizational hierarchy by referencing the "id" attribute of another User.
    pub manager: Option<Manager>,
}

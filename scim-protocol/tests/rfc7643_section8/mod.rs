// Tests JSON representations from https://datatracker.ietf.org/doc/html/rfc7643#section-8

// /// The following is a non-normative example of the minimal required SCIM representation in JSON format.
mod sec_8_1_minimal_user;

// /// The following is a non-normative example of the fully populated SCIM representation in JSON format.
mod sec_8_2_full_user;

/// The following is a non-normative example of the fully populated User using the enterprise User extension in JSON format.
mod sec_8_3_enterprise_user;

/// The following is a non-normative example of the SCIM Group representation in JSON format.
mod sec_8_4_group;

/// The following is a non-normative example of the SCIM service provider configuration representation in JSON format.
mod sec_8_5_service_provider_config;

/// The following is a non-normative example of the SCIM resource types in JSON format.
mod sec_8_6_resource_type;

// 8.7.  Schema Representation
//
// The following sections provide representations of schemas for both SCIM resources and service provider schemas.
// Note that the JSON representation has been modified for readability and to fit the specification format.

/// The following is intended as an example of the SCIM schema representation in JSON format for SCIM resources.
/// Where permitted, individual values and schema MAY change.
/// This example includes schema representations for "User", "Group", and "EnterpriseUser"; other schema representations are possible.
mod sec_8_7_1_resource_schema;

// /// The following is a representation of the SCIM schema for the fixed service provider schemas: ServiceProviderConfig, ResourceType, and Schema.
mod sec_8_7_2_service_provider_schema;

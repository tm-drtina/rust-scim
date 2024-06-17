use scim_protocol::protocol::Meta;
use scim_protocol::resource::schema::{
    Schema, SchemaAttribute, SchemaAttributeMutability, SchemaAttributeReturned,
    SchemaAttributeType, SchemaAttributeUniqueness,
};

#[test]
fn test() {
    let actual: Vec<Schema> =
        serde_json::from_str(include_str!("./sec_8_7_1_resource_schema.json")).unwrap();
    assert_eq!(expected(), actual);
}

fn expected() -> Vec<Schema> {
    vec![
        Schema {
            id: "urn:ietf:params:scim:schemas:core:2.0:User".to_string(),
            meta: Meta {
                resource_type: Some("Schema".to_string()),
                created: None,
                last_modified: None,
                location: Some(
                    "/v2/Schemas/urn:ietf:params:scim:schemas:core:2.0:User".to_string(),
                ),
                version: None,
            },
            name: Some("User".to_string()),
            description: Some("User Account".to_string()),
            attributes: vec![
                SchemaAttribute {
                    name: "userName".to_string(),
                    attribute_type: SchemaAttributeType::String,
                    multi_valued: false,
                    description: "Unique identifier for the User, typically used by the user to \
                                  directly authenticate to the service provider. Each User MUST \
                                  include a non-empty userName value.  This identifier MUST be \
                                  unique across the service provider's entire set of Users. \
                                  REQUIRED."
                        .to_string(),
                    required: true,
                    canonical_values: Default::default(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadWrite,
                    returned: SchemaAttributeReturned::Default,
                    uniqueness: SchemaAttributeUniqueness::Server,
                },
                SchemaAttribute {
                    name: "name".to_string(),
                    multi_valued: false,
                    description: "The components of the user's real name. Providers MAY return \
                                  just the full name as a single string in the formatted \
                                  sub-attribute, or they MAY return just the individual component \
                                  attributes using the other sub-attributes, or they MAY return \
                                  both.  If both variants are returned, they SHOULD be describing \
                                  the same name, with the formatted name indicating how the \
                                  component attributes should be combined."
                        .to_string(),
                    required: false,
                    canonical_values: Default::default(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadWrite,
                    returned: SchemaAttributeReturned::Default,
                    uniqueness: SchemaAttributeUniqueness::None,
                    attribute_type: SchemaAttributeType::Complex {
                        sub_attributes: vec![
                            SchemaAttribute {
                                name: "formatted".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "The full name, including all middle names, titles, \
                                              and suffixes as appropriate, formatted for display \
                                              (e.g., 'Ms. Barbara J Jensen, III')."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "familyName".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "The family name of the User, or last name in most \
                                              Western languages (e.g., 'Jensen' given the full \
                                              name 'Ms. Barbara J Jensen, III')."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "givenName".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "The given name of the User, or first name in most \
                                              Western languages (e.g., 'Barbara' given the full \
                                              name 'Ms. Barbara J Jensen, III')."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "middleName".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "The middle name(s) of the User (e.g., 'Jane' given \
                                              the full name 'Ms. Barbara J Jensen, III')."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "honorificPrefix".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "The honorific prefix(es) of the User, or title in \
                                              most Western languages (e.g., 'Ms.' given the full \
                                              name 'Ms. Barbara J Jensen, III')."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "honorificSuffix".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "The honorific suffix(es) of the User, or suffix in \
                                              most Western languages (e.g., 'III' given the full \
                                              name 'Ms. Barbara J Jensen, III')."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                        ],
                    },
                },
                SchemaAttribute {
                    name: "displayName".to_string(),
                    attribute_type: SchemaAttributeType::String,
                    multi_valued: false,
                    description: "The name of the User, suitable for display to end-users.  The \
                                  name SHOULD be the full name of the User being described, if \
                                  known."
                        .to_string(),
                    required: false,
                    canonical_values: Default::default(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadWrite,
                    returned: SchemaAttributeReturned::Default,
                    uniqueness: SchemaAttributeUniqueness::None,
                },
                SchemaAttribute {
                    name: "nickName".to_string(),
                    attribute_type: SchemaAttributeType::String,
                    multi_valued: false,
                    description: "The casual way to address the user in real life, e.g., 'Bob' or \
                                  'Bobby' instead of 'Robert'.  This attribute SHOULD NOT be used \
                                  to represent a User's username (e.g., 'bjensen' or \
                                  'mpepperidge')."
                        .to_string(),
                    required: false,
                    canonical_values: Default::default(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadWrite,
                    returned: SchemaAttributeReturned::Default,
                    uniqueness: SchemaAttributeUniqueness::None,
                },
                SchemaAttribute {
                    name: "profileUrl".to_string(),
                    attribute_type: SchemaAttributeType::Reference {
                        reference_types: vec!["external".to_string()],
                    },
                    multi_valued: false,
                    description: "A fully qualified URL pointing to a page representing the \
                                  User's online profile."
                        .to_string(),
                    required: false,
                    canonical_values: Default::default(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadWrite,
                    returned: SchemaAttributeReturned::Default,
                    uniqueness: SchemaAttributeUniqueness::None,
                },
                SchemaAttribute {
                    name: "title".to_string(),
                    attribute_type: SchemaAttributeType::String,
                    multi_valued: false,
                    description: "The user's title, such as \"Vice President.\"".to_string(),
                    required: false,
                    canonical_values: Default::default(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadWrite,
                    returned: SchemaAttributeReturned::Default,
                    uniqueness: SchemaAttributeUniqueness::None,
                },
                SchemaAttribute {
                    name: "userType".to_string(),
                    attribute_type: SchemaAttributeType::String,
                    multi_valued: false,
                    description: "Used to identify the relationship between the organization and \
                                  the user.  Typical values used might be 'Contractor', \
                                  'Employee', 'Intern', 'Temp', 'External', and 'Unknown', but \
                                  any value may be used."
                        .to_string(),
                    required: false,
                    canonical_values: Default::default(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadWrite,
                    returned: SchemaAttributeReturned::Default,
                    uniqueness: SchemaAttributeUniqueness::None,
                },
                SchemaAttribute {
                    name: "preferredLanguage".to_string(),
                    attribute_type: SchemaAttributeType::String,
                    multi_valued: false,
                    description: "Indicates the User's preferred written or spoken language.  \
                                  Generally used for selecting a localized user interface; e.g., \
                                  'en_US' specifies the language English and country US."
                        .to_string(),
                    required: false,
                    canonical_values: Default::default(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadWrite,
                    returned: SchemaAttributeReturned::Default,
                    uniqueness: SchemaAttributeUniqueness::None,
                },
                SchemaAttribute {
                    name: "locale".to_string(),
                    attribute_type: SchemaAttributeType::String,
                    multi_valued: false,
                    description: "Used to indicate the User's default location for purposes of \
                                  localizing items such as currency, date time format, or \
                                  numerical representations."
                        .to_string(),
                    required: false,
                    canonical_values: Default::default(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadWrite,
                    returned: SchemaAttributeReturned::Default,
                    uniqueness: SchemaAttributeUniqueness::None,
                },
                SchemaAttribute {
                    name: "timezone".to_string(),
                    attribute_type: SchemaAttributeType::String,
                    multi_valued: false,
                    description: "The User's time zone in the 'Olson' time zone database format, \
                                  e.g., 'America/Los_Angeles'."
                        .to_string(),
                    required: false,
                    canonical_values: Default::default(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadWrite,
                    returned: SchemaAttributeReturned::Default,
                    uniqueness: SchemaAttributeUniqueness::None,
                },
                SchemaAttribute {
                    name: "active".to_string(),
                    attribute_type: SchemaAttributeType::Boolean,
                    multi_valued: false,
                    description: "A Boolean value indicating the User's administrative status."
                        .to_string(),
                    required: false,
                    canonical_values: Default::default(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadWrite,
                    returned: SchemaAttributeReturned::Default,
                    uniqueness: Default::default(),
                },
                SchemaAttribute {
                    name: "password".to_string(),
                    attribute_type: SchemaAttributeType::String,
                    multi_valued: false,
                    description: "The User's cleartext password.  This attribute is intended to \
                                  be used as a means to specify an initial password when creating \
                                  a new User or to reset an existing User's password."
                        .to_string(),
                    required: false,
                    canonical_values: Default::default(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::WriteOnly,
                    returned: SchemaAttributeReturned::Never,
                    uniqueness: SchemaAttributeUniqueness::None,
                },
                SchemaAttribute {
                    name: "emails".to_string(),
                    attribute_type: SchemaAttributeType::Complex {
                        sub_attributes: vec![
                            SchemaAttribute {
                                name: "value".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "Email addresses for the user.  The value SHOULD be \
                                              canonicalized by the service provider, e.g., \
                                              'bjensen@example.com' instead of \
                                              'bjensen@EXAMPLE.COM'. Canonical type values of \
                                              'work', 'home', and 'other'."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "display".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "A human-readable name, primarily used for display \
                                              purposes.  READ-ONLY."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "type".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "A label indicating the attribute's function, e.g., \
                                              'work' or 'home'."
                                    .to_string(),
                                required: false,
                                canonical_values: vec![
                                    "work".to_string(),
                                    "home".to_string(),
                                    "other".to_string(),
                                ],
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "primary".to_string(),
                                attribute_type: SchemaAttributeType::Boolean,
                                multi_valued: false,
                                description: "A Boolean value indicating the 'primary' or \
                                              preferred attribute value for this attribute, e.g., \
                                              the preferred mailing address or primary email \
                                              address.  The primary attribute value 'true' MUST \
                                              appear no more than once."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                        ],
                    },
                    multi_valued: true,
                    description: "Email addresses for the user.  The value SHOULD be \
                                  canonicalized by the service provider, e.g., \
                                  'bjensen@example.com' instead of 'bjensen@EXAMPLE.COM'. \
                                  Canonical type values of 'work', 'home', and 'other'."
                        .to_string(),
                    required: false,
                    canonical_values: Default::default(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadWrite,
                    returned: SchemaAttributeReturned::Default,
                    uniqueness: SchemaAttributeUniqueness::None,
                },
                SchemaAttribute {
                    name: "phoneNumbers".to_string(),
                    attribute_type: SchemaAttributeType::Complex {
                        sub_attributes: vec![
                            SchemaAttribute {
                                name: "value".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "Phone number of the User.".to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "display".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "A human-readable name, primarily used for display \
                                              purposes.  READ-ONLY."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "type".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "A label indicating the attribute's function, e.g., \
                                              'work', 'home', 'mobile'."
                                    .to_string(),
                                required: false,
                                canonical_values: vec![
                                    "work".to_string(),
                                    "home".to_string(),
                                    "mobile".to_string(),
                                    "fax".to_string(),
                                    "pager".to_string(),
                                    "other".to_string(),
                                ],
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "primary".to_string(),
                                attribute_type: SchemaAttributeType::Boolean,
                                multi_valued: false,
                                description: "A Boolean value indicating the 'primary' or \
                                              preferred attribute value for this attribute, e.g., \
                                              the preferred phone number or primary phone number.  \
                                              The primary attribute value 'true' MUST appear no \
                                              more than once."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                        ],
                    },
                    multi_valued: true,
                    description: "Phone numbers for the User.  The value SHOULD be canonicalized \
                                  by the service provider according to the format specified in \
                                  RFC 3966, e.g., 'tel:+1-201-555-0123'. Canonical type values of \
                                  'work', 'home', 'mobile', 'fax', 'pager', and 'other'."
                        .to_string(),
                    required: false,
                    canonical_values: Default::default(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadWrite,
                    returned: SchemaAttributeReturned::Default,
                    uniqueness: SchemaAttributeUniqueness::None,
                },
                SchemaAttribute {
                    name: "ims".to_string(),
                    attribute_type: SchemaAttributeType::Complex {
                        sub_attributes: vec![
                            SchemaAttribute {
                                name: "value".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "Instant messaging address for the User.".to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "display".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "A human-readable name, primarily used for display \
                                              purposes.  READ-ONLY."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "type".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "A label indicating the attribute's function, e.g., \
                                              'aim', 'gtalk', 'xmpp'."
                                    .to_string(),
                                required: false,
                                canonical_values: vec![
                                    "aim".to_string(),
                                    "gtalk".to_string(),
                                    "icq".to_string(),
                                    "xmpp".to_string(),
                                    "msn".to_string(),
                                    "skype".to_string(),
                                    "qq".to_string(),
                                    "yahoo".to_string(),
                                ],
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "primary".to_string(),
                                attribute_type: SchemaAttributeType::Boolean,
                                multi_valued: false,
                                description: "A Boolean value indicating the 'primary' or \
                                              preferred attribute value for this attribute, e.g., \
                                              the preferred messenger or primary messenger.  The \
                                              primary attribute value 'true' MUST appear no more \
                                              than once."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                        ],
                    },
                    multi_valued: true,
                    description: "Instant messaging addresses for the User.".to_string(),
                    required: false,
                    canonical_values: Default::default(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadWrite,
                    returned: SchemaAttributeReturned::Default,
                    uniqueness: SchemaAttributeUniqueness::None,
                },
                SchemaAttribute {
                    name: "photos".to_string(),
                    attribute_type: SchemaAttributeType::Complex {
                        sub_attributes: vec![
                            SchemaAttribute {
                                name: "value".to_string(),
                                attribute_type: SchemaAttributeType::Reference {
                                    reference_types: vec!["external".to_string()],
                                },
                                multi_valued: false,
                                description: "URL of a photo of the User.".to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "display".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "A human-readable name, primarily used for display \
                                              purposes.  READ-ONLY."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "type".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "A label indicating the attribute's function, i.e., \
                                              'photo' or 'thumbnail'."
                                    .to_string(),
                                required: false,
                                canonical_values: vec![
                                    "photo".to_string(),
                                    "thumbnail".to_string(),
                                ],
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "primary".to_string(),
                                attribute_type: SchemaAttributeType::Boolean,
                                multi_valued: false,
                                description: "A Boolean value indicating the 'primary' or \
                                              preferred attribute value for this attribute, e.g., \
                                              the preferred photo or thumbnail.  The primary \
                                              attribute value 'true' MUST appear no more than \
                                              once."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                        ],
                    },
                    multi_valued: true,
                    description: "URLs of photos of the User.".to_string(),
                    required: false,
                    canonical_values: Default::default(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadWrite,
                    returned: SchemaAttributeReturned::Default,
                    uniqueness: SchemaAttributeUniqueness::None,
                },
                SchemaAttribute {
                    name: "addresses".to_string(),
                    attribute_type: SchemaAttributeType::Complex {
                        sub_attributes: vec![
                            SchemaAttribute {
                                name: "formatted".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "The full mailing address, formatted for display or \
                                              use with a mailing label.  This attribute MAY \
                                              contain newlines."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "streetAddress".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "The full street address component, which may \
                                              include house number, street name, P.O. box, and \
                                              multi-line extended street address information.  \
                                              This attribute MAY contain newlines."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "locality".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "The city or locality component.".to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "region".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "The state or region component.".to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "postalCode".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "The zip code or postal code component.".to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "country".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "The country name component.".to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "type".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "A label indicating the attribute's function, e.g., \
                                              'work' or 'home'."
                                    .to_string(),
                                required: false,
                                canonical_values: vec![
                                    "work".to_string(),
                                    "home".to_string(),
                                    "other".to_string(),
                                ],
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                        ],
                    },
                    multi_valued: true,
                    description: "A physical mailing address for this User. Canonical type values \
                                  of 'work', 'home', and 'other'.  This attribute is a complex \
                                  type with the following sub-attributes."
                        .to_string(),
                    required: false,
                    canonical_values: Default::default(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadWrite,
                    returned: SchemaAttributeReturned::Default,
                    uniqueness: SchemaAttributeUniqueness::None,
                },
                SchemaAttribute {
                    name: "groups".to_string(),
                    attribute_type: SchemaAttributeType::Complex {
                        sub_attributes: vec![
                            SchemaAttribute {
                                name: "value".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "The identifier of the User's group.".to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadOnly,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "$ref".to_string(),
                                attribute_type: SchemaAttributeType::Reference {
                                    reference_types: vec!["User".to_string(), "Group".to_string()],
                                },
                                multi_valued: false,
                                description: "The URI of the corresponding 'Group' resource to \
                                              which the user belongs."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadOnly,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "display".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "A human-readable name, primarily used for display \
                                              purposes.  READ-ONLY."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadOnly,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "type".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "A label indicating the attribute's function, e.g., \
                                              'direct' or 'indirect'."
                                    .to_string(),
                                required: false,
                                canonical_values: vec![
                                    "direct".to_string(),
                                    "indirect".to_string(),
                                ],
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadOnly,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                        ],
                    },
                    multi_valued: true,
                    description: "A list of groups to which the user belongs, either through \
                                  direct membership, through nested groups, or dynamically \
                                  calculated."
                        .to_string(),
                    required: false,
                    canonical_values: Default::default(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadOnly,
                    returned: SchemaAttributeReturned::Default,
                    uniqueness: SchemaAttributeUniqueness::None,
                },
                SchemaAttribute {
                    name: "entitlements".to_string(),
                    attribute_type: SchemaAttributeType::Complex {
                        sub_attributes: vec![
                            SchemaAttribute {
                                name: "value".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "The value of an entitlement.".to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "display".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "A human-readable name, primarily used for display \
                                              purposes.  READ-ONLY."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "type".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "A label indicating the attribute's function."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "primary".to_string(),
                                attribute_type: SchemaAttributeType::Boolean,
                                multi_valued: false,
                                description: "A Boolean value indicating the 'primary' or \
                                              preferred attribute value for this attribute.  The \
                                              primary attribute value 'true' MUST appear no more \
                                              than once."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                        ],
                    },
                    multi_valued: true,
                    description: "A list of entitlements for the User that represent a thing the \
                                  User has."
                        .to_string(),
                    required: false,
                    canonical_values: Default::default(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadWrite,
                    returned: SchemaAttributeReturned::Default,
                    uniqueness: SchemaAttributeUniqueness::None,
                },
                SchemaAttribute {
                    name: "roles".to_string(),
                    attribute_type: SchemaAttributeType::Complex {
                        sub_attributes: vec![
                            SchemaAttribute {
                                name: "value".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "The value of a role.".to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "display".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "A human-readable name, primarily used for display \
                                              purposes.  READ-ONLY."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "type".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "A label indicating the attribute's function."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "primary".to_string(),
                                attribute_type: SchemaAttributeType::Boolean,
                                multi_valued: false,
                                description: "A Boolean value indicating the 'primary' or \
                                              preferred attribute value for this attribute.  The \
                                              primary attribute value 'true' MUST appear no more \
                                              than once."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                        ],
                    },
                    multi_valued: true,
                    description: "A list of roles for the User that collectively represent who \
                                  the User is, e.g., 'Student', 'Faculty'."
                        .to_string(),
                    required: false,
                    canonical_values: Default::default(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadWrite,
                    returned: SchemaAttributeReturned::Default,
                    uniqueness: SchemaAttributeUniqueness::None,
                },
                SchemaAttribute {
                    name: "x509Certificates".to_string(),
                    attribute_type: SchemaAttributeType::Complex {
                        sub_attributes: vec![
                            SchemaAttribute {
                                name: "value".to_string(),
                                attribute_type: SchemaAttributeType::Binary,
                                multi_valued: false,
                                description: "The value of an X.509 certificate.".to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "display".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "A human-readable name, primarily used for display \
                                              purposes.  READ-ONLY."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "type".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "A label indicating the attribute's function."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "primary".to_string(),
                                attribute_type: SchemaAttributeType::Boolean,
                                multi_valued: false,
                                description: "A Boolean value indicating the 'primary' or \
                                              preferred attribute value for this attribute.  The \
                                              primary attribute value 'true' MUST appear no more \
                                              than once."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                        ],
                    },
                    multi_valued: true,
                    description: "A list of certificates issued to the User.".to_string(),
                    required: false,
                    canonical_values: Default::default(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadWrite,
                    returned: SchemaAttributeReturned::Default,
                    uniqueness: SchemaAttributeUniqueness::None,
                },
            ],
        },
        Schema {
            id: "urn:ietf:params:scim:schemas:core:2.0:Group".to_string(),
            meta: Meta {
                resource_type: Some("Schema".to_string()),
                created: None,
                last_modified: None,
                location: Some(
                    "/v2/Schemas/urn:ietf:params:scim:schemas:core:2.0:Group".to_string(),
                ),
                version: None,
            },
            name: Some("Group".to_string()),
            description: Some("Group".to_string()),
            attributes: vec![
                SchemaAttribute {
                    name: "displayName".to_string(),
                    attribute_type: SchemaAttributeType::String,
                    multi_valued: false,
                    description: "A human-readable name for the Group. REQUIRED.".to_string(),
                    required: false,
                    canonical_values: Vec::new(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadWrite,
                    returned: SchemaAttributeReturned::Default,
                    uniqueness: SchemaAttributeUniqueness::None,
                },
                SchemaAttribute {
                    name: "members".to_string(),
                    attribute_type: SchemaAttributeType::Complex {
                        sub_attributes: vec![
                            SchemaAttribute {
                                name: "value".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "Identifier of the member of this Group.".to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::Immutable,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "$ref".to_string(),
                                attribute_type: SchemaAttributeType::Reference {
                                    reference_types: vec!["User".to_string(), "Group".to_string()],
                                },
                                multi_valued: false,
                                description: "The URI corresponding to a SCIM resource that is a \
                                              member of this Group."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::Immutable,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "type".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "A label indicating the type of resource, e.g., \
                                              'User' or 'Group'."
                                    .to_string(),
                                required: false,
                                canonical_values: vec!["User".to_string(), "Group".to_string()],
                                case_exact: false,
                                mutability: SchemaAttributeMutability::Immutable,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                        ],
                    },
                    multi_valued: true,
                    description: "A list of members of the Group.".to_string(),
                    required: false,
                    canonical_values: Vec::new(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadWrite,
                    returned: Default::default(),
                    uniqueness: Default::default(),
                },
            ],
        },
        Schema {
            id: "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User".to_string(),
            meta: Meta {
                resource_type: Some("Schema".to_string()),
                created: None,
                last_modified: None,
                location: Some(
                    "/v2/Schemas/urn:ietf:params:scim:schemas:extension:enterprise:2.0:User"
                        .to_string(),
                ),
                version: None,
            },
            name: Some("EnterpriseUser".to_string()),
            description: Some("Enterprise User".to_string()),
            attributes: vec![
                SchemaAttribute {
                    name: "employeeNumber".to_string(),
                    attribute_type: SchemaAttributeType::String,
                    multi_valued: false,
                    description: "Numeric or alphanumeric identifier assigned to a person, \
                                  typically based on order of hire or association with an \
                                  organization."
                        .to_string(),
                    required: false,
                    canonical_values: Default::default(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadWrite,
                    returned: SchemaAttributeReturned::Default,
                    uniqueness: SchemaAttributeUniqueness::None,
                },
                SchemaAttribute {
                    name: "costCenter".to_string(),
                    attribute_type: SchemaAttributeType::String,
                    multi_valued: false,
                    description: "Identifies the name of a cost center.".to_string(),
                    required: false,
                    canonical_values: Default::default(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadWrite,
                    returned: SchemaAttributeReturned::Default,
                    uniqueness: SchemaAttributeUniqueness::None,
                },
                SchemaAttribute {
                    name: "organization".to_string(),
                    attribute_type: SchemaAttributeType::String,
                    multi_valued: false,
                    description: "Identifies the name of an organization.".to_string(),
                    required: false,
                    canonical_values: Default::default(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadWrite,
                    returned: SchemaAttributeReturned::Default,
                    uniqueness: SchemaAttributeUniqueness::None,
                },
                SchemaAttribute {
                    name: "division".to_string(),
                    attribute_type: SchemaAttributeType::String,
                    multi_valued: false,
                    description: "Identifies the name of a division.".to_string(),
                    required: false,
                    canonical_values: Default::default(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadWrite,
                    returned: SchemaAttributeReturned::Default,
                    uniqueness: SchemaAttributeUniqueness::None,
                },
                SchemaAttribute {
                    name: "department".to_string(),
                    attribute_type: SchemaAttributeType::String,
                    multi_valued: false,
                    description: "Identifies the name of a department.".to_string(),
                    required: false,
                    canonical_values: Default::default(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadWrite,
                    returned: SchemaAttributeReturned::Default,
                    uniqueness: SchemaAttributeUniqueness::None,
                },
                SchemaAttribute {
                    name: "manager".to_string(),
                    attribute_type: SchemaAttributeType::Complex {
                        sub_attributes: vec![
                            SchemaAttribute {
                                name: "value".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "The id of the SCIM resource representing the User's \
                                              manager.  REQUIRED."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "$ref".to_string(),
                                attribute_type: SchemaAttributeType::Reference {
                                    reference_types: vec!["User".to_string()],
                                },
                                multi_valued: false,
                                description: "The URI of the SCIM resource representing the \
                                              User's manager.  REQUIRED."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadWrite,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                            SchemaAttribute {
                                name: "displayName".to_string(),
                                attribute_type: SchemaAttributeType::String,
                                multi_valued: false,
                                description: "The displayName of the User's manager. OPTIONAL and \
                                              READ-ONLY."
                                    .to_string(),
                                required: false,
                                canonical_values: Default::default(),
                                case_exact: false,
                                mutability: SchemaAttributeMutability::ReadOnly,
                                returned: SchemaAttributeReturned::Default,
                                uniqueness: SchemaAttributeUniqueness::None,
                            },
                        ],
                    },
                    multi_valued: false,
                    description: "The User's manager.  A complex type that optionally allows \
                                  service providers to represent organizational hierarchy by \
                                  referencing the 'id' attribute of another User."
                        .to_string(),
                    required: false,
                    canonical_values: Default::default(),
                    case_exact: false,
                    mutability: SchemaAttributeMutability::ReadWrite,
                    returned: SchemaAttributeReturned::Default,
                    uniqueness: SchemaAttributeUniqueness::None,
                },
            ],
        },
    ]
}

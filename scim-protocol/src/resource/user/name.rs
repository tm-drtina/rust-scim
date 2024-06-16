use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Name {
    /// The full name, including all middle names, titles, and suffixes as appropriate, formatted for display (e.g., "Ms. Barbara Jane Jensen, III").
    pub formatted: Option<String>,

    /// The family name of the User, or last name in most Western languages (e.g., "Jensen" given the full name "Ms. Barbara Jane Jensen, III").
    pub family_name: Option<String>,

    /// The given name of the User, or first name in most Western languages (e.g., "Barbara" given the full name "Ms. Barbara Jane Jensen, III").
    pub given_name: Option<String>,

    /// The middle name(s) of the User (e.g., "Jane" given the full name "Ms. Barbara Jane Jensen, III").
    pub middle_name: Option<String>,

    /// The honorific prefix(es) of the User, or title in most Western languages (e.g., "Ms." given the full name "Ms. Barbara Jane Jensen, III").
    pub honorific_prefix: Option<String>,

    /// The honorific suffix(es) of the User, or suffix in most Western languages (e.g., "III" given the full name "Ms. Barbara Jane Jensen, III").
    pub honorific_suffix: Option<String>,
}

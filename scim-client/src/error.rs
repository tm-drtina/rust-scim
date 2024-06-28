use scim_protocol::protocol::SchemaMismatch;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ScimError>;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ScimError {
    #[error(
        "schema mismatch (got extra schema definitions {extra:?}, missing definitions {missing:?})"
    )]
    SchemaMismatch {
        missing: Vec<String>,
        extra: Vec<String>,
    },

    #[error(transparent)]
    HttpError(#[from] reqwest::Error),

    #[error("unknown error")]
    Unknown,
}

impl<'a> From<SchemaMismatch<'a>> for ScimError {
    fn from(value: SchemaMismatch) -> Self {
        ScimError::SchemaMismatch {
            missing: value.missing.into_iter().map(String::from).collect(),
            extra: value.extra.into_iter().map(String::from).collect(),
        }
    }
}

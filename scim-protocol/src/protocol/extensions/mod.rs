mod generate;

use crate::generate_extension;

pub trait Extensions {
    const SCHEMA: &'static [&'static str];
}

generate_extension!(
    extension NoExtensions {}
);

mod generate;
mod no_extensions;

pub use no_extensions::NoExtensions;

pub trait Extensions {
    const SCHEMA: &'static [&'static str];
}

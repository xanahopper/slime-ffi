pub use slime_ffi::runtime::Language;

pub mod types;
pub mod meta;

pub struct Project {
    pub name: String,
    pub addon_types: Vec<Language>,
}

pub mod scheme;

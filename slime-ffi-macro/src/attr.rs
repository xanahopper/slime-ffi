use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use slime_ffi_gen::Language;

use crate::symbol::Symbol;

pub struct Attr<T> {
    pub name: Symbol,
    pub tokens: TokenStream,
    pub value: Option<T>,
}

pub struct Name {
    renames: BTreeMap<Language, String>
}
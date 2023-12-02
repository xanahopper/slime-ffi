use std::fmt::Display;

use syn::{Ident, Path};

#[derive(Clone, Copy)]
pub struct Symbol(&'static str);

pub const SLIME: Symbol = Symbol("slime");
pub const PACKAGE_NAME: Symbol = Symbol("package_name");
pub const MODULEMAP_NAME: Symbol = Symbol("modulemap_name");
pub const LIBRARY_NAME: Symbol = Symbol("library_name");
pub const DISPATCHER_ENABLE: Symbol = Symbol("dispatcher");
pub const ENTRY: Symbol = Symbol("ffi_entry");
pub const RENAME: Symbol = Symbol("rename");
pub const IGNORE: Symbol = Symbol("ignore");
pub const DOC: Symbol = Symbol("doc");

pub const ENTRY_JVM: Symbol = Symbol("jvm");

impl PartialEq<Symbol> for Ident {
    fn eq(&self, other: &Symbol) -> bool {
        self == other.0
    }
}

impl PartialEq<Symbol> for &Ident {
    fn eq(&self, other: &Symbol) -> bool {
        *self == other.0
    }
}

impl PartialEq<Symbol> for Path {
    fn eq(&self, other: &Symbol) -> bool {
        self.is_ident(other.0)
    }
}

impl PartialEq<Symbol> for &Path {
    fn eq(&self, other: &Symbol) -> bool {
        self.is_ident(other.0)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}
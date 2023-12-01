use syn::{parse::{Parse, ParseStream}, ItemMod, Error, AttrStyle, ItemStruct, visit::Visit, Ident, spanned::Spanned, Lit, ExprAssign, Token, Meta, punctuated::Punctuated, Expr, ExprLit};

use crate::symbol::{DISPATCHER_ENABLE, PACKAGE_NAME, MODULEMAP_NAME, LIBRARY_NAME, ENTRY, Symbol, ENTRY_JVM};

use super::{Item, name::Name, ItemAttr};

/// An exported module, which can and only can have one in a crate.
pub struct Module {
    pub attrs: Vec<Attr>,
    pub items: Vec<ModuleItem>,
}

impl Parse for Module {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        
        let r#mod: ItemMod = input.parse().map_err(|e| {
            Error::new(
                e.span(),
                "`slime` attribute is supported on mod only.",
            )
        })?;

        let mut visitor = ModuleVisitor::new(&r#mod.ident);
        visitor.visit_item_mod(&r#mod);
        Ok(visitor.into())
    }
}

pub struct ModuleItem {
    pub name: Name,
    pub item: Item,
    pub original: syn::Item,
}

struct ModuleVisitor<'a> {
    pub ident: &'a Ident,
    pub attrs: Vec<Attr>,
    pub struts: Vec<&'a ItemStruct>,
}

impl<'a> Visit<'a> for ModuleVisitor<'a> {
    fn visit_item_mod(&mut self, item_mod: &'a ItemMod) {
        for attr in &item_mod.attrs {
            self.visit_attribute(attr);
        }
        if let Some((_, items)) = &item_mod.content {
            for item in items {
                self.visit_item(item);
            }
        }
    }

    fn visit_attribute(&mut self, attr: &'a syn::Attribute) {
        self.attrs.push(Attr::parse_ast(&attr).unwrap())
    }
}

impl<'a> From<ModuleVisitor<'a>> for Module {
    fn from(value: ModuleVisitor) -> Self {
        Module { attrs: value.attrs, items: vec![] }
    }
}

impl<'a> ModuleVisitor<'a> {
    fn new(ident: &'a Ident) -> Self {
        Self { ident, attrs: vec![], struts: vec![] }
    }
}

/// Config for exported module by `syn::Attribute`
pub enum Attr {
    /// Package name for Java/Kotlin/Jvm
    PackageName(String),
    /// Modulemap name for Swift
    ModulemapName(String),
    /// Default library name (`*.so`) for loading
    LibraryName(String),
    /// Entry config for different language
    Entry(Vec<EntryAttr>),
    /// Enable for the dispatcher
    Dispatcher,
}

impl Attr {
    pub fn parse_ast(attr: &syn::Attribute) -> syn::Result<Self> {
        match &attr.meta {
            Meta::Path(pat) => if pat == DISPATCHER_ENABLE {
                Ok(Attr::Dispatcher)
            } else {
                Err(Error::new(attr.span(), format!("unknown attribute: {:?}", pat.get_ident())))
            },
            Meta::List(lit) => {
                let path = &lit.path;
                if path == ENTRY {
                    let entries = attr.parse_args_with(Punctuated::<EntryAttr, Token![,]>::parse_terminated)?;
                    Ok(Attr::Entry(entries.into_iter().collect()))
                } else {
                    Err(Error::new(attr.span(), format!("unknown attribute: {:?}", path.get_ident().map(|s| s.to_string()))))
                }
            },
            Meta::NameValue(nv) => {
                let path = &nv.path;
                if let Expr::Lit(ExprLit { ref lit, .. }) = nv.value {
                    if path == PACKAGE_NAME {
                        if let Lit::Str(package_name) = lit {
                            Ok(Attr::PackageName(package_name.value().to_owned()))
                        } else {
                            Err(Error::new(attr.span(), format!("{} require a string value", PACKAGE_NAME)))
                        }
                    } else if path == MODULEMAP_NAME {
                        if let Lit::Str(modulemap_name) = lit {
                            Ok(Attr::ModulemapName(modulemap_name.value()))
                        } else {
                            Err(Error::new(attr.span(), format!("{} require a string value", MODULEMAP_NAME)))
                        }
                    } else if path == LIBRARY_NAME {
                        if let Lit::Str(library_name) = lit {
                            Ok(Attr::LibraryName(library_name.value().to_owned()))
                        } else {
                            Err(Error::new(attr.span(), format!("{} require a string value", LIBRARY_NAME)))
                        }
                    } else {
                        Err(Error::new(attr.span(), format!("unknown attribute: {:?}", path.get_ident())))
                    }
                } else {
                    Err(Error::new(attr.span(), format!("{:?} require a value", path.get_ident())))
                }
                
            },
        }
    }
}

pub enum EntryAttr {
    JvmEntry(JNIEntry),
}

impl Parse for EntryAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Meta::NameValue(nv)= input.parse()? {
            if nv.path == ENTRY_JVM {
                if let Expr::Lit(ExprLit { lit: Lit::Str(entry_name), ..}) = nv.value {
                    Ok(EntryAttr::JvmEntry(JNIEntry(entry_name.value())))
                } else {
                    Err(input.error("entry require a string value"))
                }
            } else {
                Err(input.error(format!("unknown language type: {:?}", nv.path.get_ident())))
            }
        } else {
            Err(input.error("require a name-value expression"))
        }
    }
}

pub struct JNIEntry(pub String);

#[cfg(test)]
mod test {
    use syn::ItemMod;

    use crate::types::{EntryAttr, JNIEntry};

    use super::Attr;


    #[test]
    fn module_attributes_test() {
        let content = quote::quote! {
            mod ffi {
                #![package_name = "com.slime.ffi"]
                #![modulemap_name = "SlimeFFI"]
                #![library_name = "slime"]
                #![dispatcher]
                #![ffi_entry(jvm = "on_jvm_loaded")]
            }
        };
        let raw_attrs: ItemMod = syn::parse2(content).unwrap();
        let mut has_dispatcher = false;
        for raw_attr in &raw_attrs.attrs {
            let attr = Attr::parse_ast(raw_attr).unwrap();
            match attr {
                Attr::PackageName(name) => assert_eq!(name, "com.slime.ffi"),
                Attr::ModulemapName(name) => assert_eq!(name, "SlimeFFI"),
                Attr::LibraryName(name) => assert_eq!(name, "slime"),
                Attr::Entry(entries) => {
                    assert_eq!(entries.len(), 1);
                    assert!(matches!(entries.get(0).unwrap(), EntryAttr::JvmEntry(JNIEntry(_))));
                    match entries.get(0).unwrap() {
                        EntryAttr::JvmEntry(JNIEntry(name)) => {
                            assert_eq!(name, "on_jvm_loaded");
                        }
                    }
                },
                Attr::Dispatcher => {
                    has_dispatcher = true
                },
            }
        }
        assert_eq!(has_dispatcher, true);
    }

    #[test]
    fn model_parse_test() {
        use quote::quote;
        use super::Module;

        let content = quote! {
            mod ffi {
                #![package_name = "com.slime.ffi"]
                #![modulemap_name = "SlimeFFI"]
                #![library_name = "slime"]
                #![dispatcher]
                #![ffi_entry(jvm = "on_jvm_loaded")]
            }
        };

        let module: Module = syn::parse2(content).unwrap();
        let mut has_dispatcher = false;
        for attr in &module.attrs {
            match attr {
                Attr::PackageName(name) => assert_eq!(name, "com.slime.ffi"),
                Attr::ModulemapName(name) => assert_eq!(name, "SlimeFFI"),
                Attr::LibraryName(name) => assert_eq!(name, "slime"),
                Attr::Entry(entries) => {
                    assert_eq!(entries.len(), 1);
                    assert!(matches!(entries.get(0).unwrap(), EntryAttr::JvmEntry(JNIEntry(_))));
                    match entries.get(0).unwrap() {
                        EntryAttr::JvmEntry(JNIEntry(name)) => {
                            assert_eq!(name, "on_jvm_loaded");
                        }
                    }
                },
                Attr::Dispatcher => {
                    has_dispatcher = true
                },
            }
        }
        assert_eq!(has_dispatcher, true);
    }
}
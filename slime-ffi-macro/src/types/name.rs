use std::{collections::BTreeMap, str::FromStr};

use slime_ffi_gen::Language;
use syn::{Expr, ExprLit, Lit, LitStr, ExprAssign, punctuated::Punctuated, Token};

use crate::symbol::RENAME;

use super::ItemAttr;

/// Specify a name(Ident), in different language, such as
/// function, struct, field, method, etc.
/// 
/// # Examples
/// ```rust
/// pub struct User {
///     #[rename("account")]
///     pub username: String,
///     #[rename(
///         jvm = "nickname",
///         swift = "avatar"
///     )]
///     pub name: String,
/// }
/// ````
pub struct Name {
    pub ident: syn::Ident,
    pub rename_rules: Option<RenameRules>,
}

pub struct RenameRules {
    pub rename_all: Option<String>,
    pub dialects: BTreeMap<Language, String>
}

impl Name {
    pub fn parse_ast(ident: &syn::Ident, attrs: &[ItemAttr]) -> syn::Result<Self> {
        let ident = ident.clone();
        let mut rename_rules = None;
        for attr in attrs {
            if let ItemAttr::Custom(raw_attr) = attr {
                if let syn::Meta::List(meta) = &raw_attr.meta {
                    if meta.path == RENAME {
                        if rename_rules.is_some() {
                            return Err(syn::Error::new(ident.span(), "duplicate rename attribute"));
                        }
                        rename_rules = Some(RenameRules::parse_ast(&meta)?);
                    }
                }
            }
        }
        Ok(Name {
            ident,
            rename_rules,
        })
    }
}

impl RenameRules {
    pub fn parse_ast(meta_list: &syn::MetaList) -> syn::Result<Self> {
        let mut rename_all = None;
        let mut dialects = BTreeMap::new();
        let args = meta_list.parse_args_with(Punctuated::<Expr, Token![,]>::parse_terminated)?;
        for arg in args {
            match arg {
                Expr::Lit(ExprLit { lit: Lit::Str(name), .. }) => if rename_all.is_some() {
                    return Err(syn::Error::new_spanned(meta_list, format!("Single rename parameter require single string value")))
                } else {
                    rename_all = Some(name.value());
                },
                Expr::Assign(ExprAssign { left, right, .. }) => {
                    if let Expr::Path(path) = *left {
                        if let Expr::Lit(ExprLit { lit: Lit::Str(name), .. }) = *right {
                            let ident = path.path.require_ident()?;
                            if let Ok(language) = Language::from_str(ident.to_string().as_str()) {
                                dialects.insert(language, name.value());
                            } else {
                                return Err(syn::Error::new_spanned(meta_list, format!("Unknown language: {}", ident)))
                            }
                        } else {
                            return Err(syn::Error::new_spanned(right, format!("rename dialect value should be string value")))
                        }
                    } else {
                        return Err(syn::Error::new_spanned(left, "require a `language = \"name\"` list"));
                    }
                }
                _ => return Err(syn::Error::new_spanned(meta_list, "invalid rename attribute"))
            }
        }
        Ok(RenameRules {
            rename_all,
            dialects,
        })
    }
}

#[cfg(test)]
mod test {
    use syn::Meta;

    use super::*;

    #[test]
    fn test_parse_rename_rules() {
        let attr: syn::Attribute = syn::parse_quote! {
            #[rename(
                #[cfg(feature = "jvm")]
                jvm = "nickname",
                c = "avatar",
            )]
        };
        if let Meta::List(meta_list) = &attr.meta {
            let rules = RenameRules::parse_ast(&meta_list).unwrap();
            assert_eq!(rules.rename_all, None);
            assert_eq!(rules.dialects.len(), 2);
            #[cfg(feature = "jvm")]
            assert_eq!(rules.dialects.get(&Language::Jvm), Some(&"nickname".to_owned()));
            assert_eq!(rules.dialects.get(&Language::C), Some(&"avatar".to_owned()));
        }
    }

    #[test]
    fn test_parse_rename_all() {
        let attr: syn::Attribute = syn::parse_quote! {
            #[rename("username")]
        };
        if let Meta::List(meta_list) = &attr.meta {
            let rules = RenameRules::parse_ast(&meta_list).unwrap();
            assert_eq!(rules.rename_all, Some("username".to_owned()));
            assert_eq!(rules.dialects.len(), 0);
        }
    }
}

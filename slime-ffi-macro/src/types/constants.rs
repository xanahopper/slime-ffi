use std::str::FromStr;

use syn::{Error, Expr};

use super::{name::Name, ItemAttr};

pub struct ConstantItem {
    pub name: Name,
    pub value: ConstantValue,
}

impl ConstantItem {
    pub fn parse_ast(item: &syn::ItemConst) -> syn::Result<ConstantItem> {
        let attrs: Vec<ItemAttr> = item.attrs.iter().map(|attr| ItemAttr::parse_ast(attr)).collect::<syn::Result<Vec<_>>>()?;
        let ident = item.ident.clone();
        let const_ty = ConstantType::try_from(&*item.ty)?;
        let value = match item.expr.as_ref() {
            Expr::Lit(lit) => ConstantValue::parse_ast(&const_ty, &lit.lit)?,
            _ => todo!(),
        };
        let name = Name::parse_ast(&ident, &attrs)?;

        Ok(ConstantItem { name, value })

    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConstantType {
    Int8,
    Int16,
    Int32,
    Int64,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Bool,
    Float,
    Double,
    String,
}

impl FromStr for ConstantType {
    type Err = syn::Error;

    fn from_str(s: &str) -> Result<Self, syn::Error> {
        match s {
            "i8" => Ok(ConstantType::Int8),
            "i16" => Ok(ConstantType::Int16),
            "i32" => Ok(ConstantType::Int32),
            "i64" => Ok(ConstantType::Int64),
            "u8" => Ok(ConstantType::Uint8),
            "u16" => Ok(ConstantType::Uint16),
            "u32" => Ok(ConstantType::Uint32),
            "u64" => Ok(ConstantType::Uint64),
            "bool" => Ok(ConstantType::Bool),
            "f32" => Ok(ConstantType::Float),
            "f64" => Ok(ConstantType::Double),
            "string" => Ok(ConstantType::String),
            _ => Err(Error::new_spanned(s, format!("Unknown constant type: {}", s)))
        }
    }
}

impl TryFrom<&syn::Type> for ConstantType {
    type Error = syn::Error;

    fn try_from(value: &syn::Type) -> Result<Self, syn::Error> {
        match value {
            syn::Type::Path(pat) => ConstantType::from_str(pat.path.require_ident()?.to_string().as_str()),
            _ => Err(syn::Error::new_spanned(value, "Invalid constant type")),
        }
    }
}

#[derive(Debug)]
pub enum ConstantValue {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Bool(bool),
    Float(f32),
    Double(f64),
    String(String),
}

impl ConstantValue {
    pub fn parse_ast(ty: &ConstantType, lit: &syn::Lit) -> syn::Result<Self> {
        use syn::Lit;
        match lit {
            Lit::Str(s) => if matches!(ty, ConstantType::String) {
                Ok(ConstantValue::String(s.value()))
            } else {
                Err(syn::Error::new_spanned(lit, format!("Missmatched constant type and value: {:?}", ty)))
            }
            Lit::Byte(b) => if matches!(ty, ConstantType::Uint8) {
                Ok(ConstantValue::Uint8(b.value() as u8))
            } else {
                Err(syn::Error::new_spanned(lit, format!("Missmatched constant type and value: {:?}", ty)))
            },
            Lit::Char(c) => if matches!(ty, ConstantType::Uint8) {
                Ok(ConstantValue::Uint8(c.value() as u8))
            } else {
                Err(syn::Error::new_spanned(lit, format!("Missmatched constant type and value: {:?}", ty)))
            }
            Lit::Int(n) => {
                let value_ty = if let Ok(decl_ty) = ConstantType::from_str(n.suffix()) {
                    if decl_ty != *ty {
                        return Err(syn::Error::new_spanned(lit, format!("Missmatched constant type and value: {:?}", ty)));
                    }
                    ty
                } else if n.suffix().is_empty() {
                    ty
                } else {
                    return Err(syn::Error::new_spanned(lit, format!("Unsupported constant value: {:?}", n.token())))
                };
                match value_ty {
                    ConstantType::Int8 => Ok(ConstantValue::Int8(n.base10_parse()?)),
                    ConstantType::Int16 => Ok(ConstantValue::Int16(n.base10_parse()?)),
                    ConstantType::Int32 => Ok(ConstantValue::Int32(n.base10_parse()?)),
                    ConstantType::Int64 => Ok(ConstantValue::Int64(n.base10_parse()?)),
                    ConstantType::Uint8 => Ok(ConstantValue::Uint8(n.base10_parse()?)),
                    ConstantType::Uint16 => Ok(ConstantValue::Uint16(n.base10_parse()?)),
                    ConstantType::Uint32 => Ok(ConstantValue::Uint32(n.base10_parse()?)),
                    ConstantType::Uint64 => Ok(ConstantValue::Uint64(n.base10_parse()?)),
                    ConstantType::Float => Ok(ConstantValue::Float(n.base10_parse()?)),
                    ConstantType::Double => Ok(ConstantValue::Double(n.base10_parse()?)),
                    _ => Err(syn::Error::new_spanned(lit, format!("Missmatched constant type and value: {:?}", ty)))
                }
            },
            Lit::Float(f) => {
                let value_ty = if let Ok(decl_ty) = ConstantType::from_str(f.suffix()) {
                    if decl_ty != *ty {
                        return Err(syn::Error::new_spanned(lit, format!("Missmatched constant type and value: {:?}", ty)));
                    }
                    ty
                } else if f.suffix().is_empty() {
                    ty
                } else {
                    return Err(syn::Error::new_spanned(lit, format!("Unsupported constant value: {:?}", f.token())))
                };
                match value_ty {
                    ConstantType::Float => Ok(ConstantValue::Float(f.base10_parse()?)),
                    ConstantType::Double => Ok(ConstantValue::Double(f.base10_parse()?)),
                    _ => Err(syn::Error::new_spanned(lit, format!("Missmatched constant type and value: {:?}", ty)))
                }
            },
            Lit::Bool(b) => {
                if !matches!(ty, ConstantType::Bool) {
                    return Err(syn::Error::new_spanned(lit, format!("Missmatched constant type and value: {:?}", ty)));
                }
                Ok(ConstantValue::Bool(b.value()))
            },
            _ => Err(syn::Error::new_spanned(lit, format!("Unsupported constant value"))),
            
        }
    }
}

#[cfg(test)]
mod test {
    use syn::ItemMod;

    use crate::types::constants::ConstantValue;

    use super::ConstantItem;


    #[test]
    fn test_constant_value() {
        let content: ItemMod = syn::parse_quote! {
            mod constants {
                const int8: i8 = 42;
                const int16: i16 = 42;
                const int32: i32 = 42;
                const int64: i64 = 42;
                const uint8: u8 = 42;
                const uint16: u16 = 42;
                const uint32: u32 = 42;
                const uint64: u64 = 42;
                const float: f32 = 3.14;
                const double: f64 = 3.14;
                const enable: bool = true;
            }
        };

        if let Some((_, items)) = content.content {
            let contants = items.iter().map(|item| match item {
                syn::Item::Const(item_const) => ConstantItem::parse_ast(item_const),
                _ => Err(syn::Error::new_spanned(item, "Unsupported constant item")),
            })
            .collect::<Result<Vec<_>, syn::Error>>()
            .unwrap();
            assert_eq!(contants.len(), 11);

            assert_eq!(contants[0].name.ident.to_string(), "int8");
            assert!(matches!(contants[0].value, ConstantValue::Int8(42)));

            assert_eq!(contants[1].name.ident.to_string(), "int16");
            assert!(matches!(contants[1].value, ConstantValue::Int16(42)));
            
            assert_eq!(contants[2].name.ident.to_string(), "int32");
            assert!(matches!(contants[2].value, ConstantValue::Int32(42)));

            assert_eq!(contants[3].name.ident.to_string(), "int64");
            assert!(matches!(contants[3].value, ConstantValue::Int64(42)));

            assert_eq!(contants[4].name.ident.to_string(), "uint8");
            assert!(matches!(contants[4].value, ConstantValue::Uint8(42)));

            assert_eq!(contants[5].name.ident.to_string(), "uint16");
            assert!(matches!(contants[5].value, ConstantValue::Uint16(42)));

            assert_eq!(contants[6].name.ident.to_string(), "uint32");
            assert!(matches!(contants[6].value, ConstantValue::Uint32(42)));

            assert_eq!(contants[7].name.ident.to_string(), "uint64");
            assert!(matches!(contants[7].value, ConstantValue::Uint64(42)));

            assert_eq!(contants[8].name.ident.to_string(), "float");
            assert!(matches!(contants[8].value, ConstantValue::Float(_)));
            if let ConstantValue::Float(n) = contants[8].value {
                assert_eq!(n, 3.14f32);
            }

            assert_eq!(contants[9].name.ident.to_string(), "double");
            assert!(matches!(contants[9].value, ConstantValue::Double(_)));
            if let ConstantValue::Double(n) = contants[9].value {
                assert_eq!(n, 3.14f64);
            }

            assert_eq!(contants[10].name.ident.to_string(), "enable");
            assert!(matches!(contants[10].value, ConstantValue::Bool(true)));
            
        }
    }
}

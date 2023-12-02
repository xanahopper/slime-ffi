mod transformer;

mod primitive;

pub use primitive::*;

mod r#enum;

use proc_macro2::Ident;
pub use r#enum::*;

mod module;

pub use module::*;
use slime_ffi_gen::Language;
use syn::{Expr, ExprLit, Lit};

use crate::symbol::{IGNORE, RENAME, DOC};

use self::constants::{ConstantValue, ConstantItem};

mod constants {
    use super::name::Name;

    pub struct ConstantItem {
        pub name: Name,
        pub value: ConstantValue,
    }
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
        pub fn parse_ast(lit: &syn::Lit) -> syn::Result<Self> {
            todo!()
        }
    }
}

pub struct StructItem {
    pub name: String,
    pub fields: Vec<Field>,
}

pub struct FnItem {
    pub name: String,
    pub params: Vec<Field>,
    pub return_type: Type,
    pub error_type: Type,
}

pub type Method = FnItem;

pub enum Accessor {
    Getter,
    Setter,
}

pub enum Member {
    Variable(Field),
    Property {
        field: Field,
        accessors: Vec<Accessor>,
    },
    Constant{
        name: String,
        value: ConstantValue,
    },
}

pub struct ClassItem {
    pub name: String,
    pub methods: Vec<Method>,
    pub ctors: Vec<FnItem>,
    pub fields: Vec<Member>,
}

pub struct InterfaceItem {
    pub name: String,
    pub methods: Vec<Method>,
}

pub enum Type {
    /* Builtin */
    Void,
    Primitive(PrimitiveType),
    String,
    Bytes,

    /* Custom */
    Enum(EnumItem),
    Model(StructItem),
    Class(ClassItem),
    Interface(InterfaceItem),

    /* Collections */
    Option(Box<Type>),
    List(Box<Type>),
    Map {
        key: Box<Type>,
        value: Box<Type>,
    }
}

pub struct Field {
    pub name: String,
    pub r#type: Type,
    pub attrs: FieldAttr,
}

pub struct FieldAttr {

}

pub struct Constant {
    pub name: Ident,
    pub value: ConstantValue,
}

pub struct DependencyItem {
    pub extern_name: String,
}

pub struct External {
    pub r#type: ExternalItem,
    pub path: Path,
}

pub struct Path {
    pub segments: Vec<String>,
    pub last: Option<String>,
}

pub enum ExternalItem {
    FnItem(FnItem),
    ModelItem(StructItem),
    ClassItem(ClassItem),
    InterfaceItem(InterfaceItem),
}

pub struct ModItem {
    pub name: String,
    pub items: Vec<Item>,
}

pub struct UseItem {
    pub path: Path,
}

pub struct ImplItem {
    pub class_name: String,
    pub interface_name: String,
}
pub enum Item {
    FnItem(FnItem),
    ConstItem(ConstantItem),
    StructItem(StructItem),
    ClassItem(ClassItem),
    InterfaceItem(InterfaceItem),
    DependencyItem(DependencyItem),
    ExternalItem(ExternalItem),
    UseItem(UseItem),
    ModItem(Box<ModItem>),
    ImplItem(ImplItem),
}

pub enum ItemAttr {
    Ignore,
    Comment(String),
    Custom(syn::Attribute),
}

impl ItemAttr {
    pub fn parse_ast(attr: &syn::Attribute) -> syn::Result<Self> {
        match &attr.meta {
            syn::Meta::Path(path) if path == IGNORE => Ok(ItemAttr::Ignore),
            syn::Meta::NameValue(nv) if nv.path == DOC => if let Expr::Lit(ExprLit { lit: Lit::Str(comment), ..}) = &nv.value {
                Ok(ItemAttr::Comment(comment.value()))
            } else {
                Err(syn::Error::new_spanned(nv, "Comment meets non-string value"))
            },
            _ => Ok(ItemAttr::Custom(attr.clone()))
        }
    }
}

mod name;

pub use name::*;

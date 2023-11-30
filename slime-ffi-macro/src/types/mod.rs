use serde::{Serialize, Deserialize};
mod transformer;

mod primitive;

pub use primitive::*;

mod r#enum;

pub use r#enum::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelItem {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FnItem {
    pub name: String,
    pub params: Vec<Field>,
    pub return_type: Type,
    pub error_type: Type,
}

pub type Method = FnItem;

#[derive(Debug, Serialize, Deserialize)]
pub enum Accessor {
    Getter,
    Setter,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub enum Property {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct AdvanceField {
    pub property: Property,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassItem {
    pub name: String,
    pub methods: Vec<Method>,
    pub ctors: Vec<FnItem>,
    pub fields: Vec<AdvanceField>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InterfaceItem {
    pub name: String,
    pub methods: Vec<Method>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    /* Builtin */
    Void,
    Primitive(PrimitiveType),
    String,
    Bytes,

    /* Custom */
    Enum(EnumItem),
    Model(ModelItem),
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub r#type: Type,
    pub attrs: FieldAttr,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldAttr {

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Constant {
    pub name: String,
    pub value: ConstantValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyItem {
    pub extern_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct External {
    pub r#type: ExternalItem,
    pub path: Path,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Path {
    pub segments: Vec<String>,
    pub last: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExternalItem {
    FnItem(FnItem),
    ModelItem(ModelItem),
    ClassItem(ClassItem),
    InterfaceItem(InterfaceItem),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModItem {
    pub name: String,
    pub items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UseItem {
    pub path: Path,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImplItem {
    pub class_name: String,
    pub interface_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Item {
    FnItem(FnItem),
    ModelItem(ModelItem),
    ClassItem(ClassItem),
    InterfaceItem(InterfaceItem),
    DependencyItem(DependencyItem),
    ExternalItem(ExternalItem),
    UseItem(UseItem),
    ModItem(Box<ModItem>),
    ImplItem(ImplItem),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageAttribute {
    pub package_name: Option<String>,
    pub modulemap_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ItemAttribute {
    Ignore,
    Rename(String),
    Comment(String),
}
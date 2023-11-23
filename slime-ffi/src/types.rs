use syn::parse::Parser;

pub enum PrimitiveType {
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
}

pub enum EnumVariantType {
    Primitive,
    Unamed(Vec<Type>),
    Named(Vec<Field>),
}

pub struct EnumVariant {
    pub name: String,
    pub variant: EnumVariantType,
}

pub struct Model {
    pub name: String,
    pub fields: Vec<Field>,
}

pub struct Func {
    pub name: String,
    pub params: Vec<Field>,
}

pub type Method = Func;

pub enum Accessor {
    Getter,
    Setter,
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

pub enum FieldAccessor {
    Variable,
    Property(Vec<Accessor>),
    Constant(ConstantValue),
}

pub struct AdvanceField {
    pub field: Field,
    pub accessor: FieldAccessor,
}

pub struct Class {
    pub name: String,
    pub methods: Vec<Method>,
    pub ctors: Vec<Func>,
    pub fields: Vec<AdvanceField>,
}

pub struct Interface {
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
    Enum(Vec<EnumVariant>),
    Model(Model),
    Class(Class),
    Interface(Interface),

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
}

pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>
}

pub struct Constant {
    pub name: String,
    pub r#type: Type,
    pub value: ConstantValue,
}

pub struct Dependency {

}

pub struct External {
    pub item_type: Item,
}

pub enum Item {
    FnItem(Func),
    ModelItem(Model),
    ClassItem(Class),
    InterfaceItem(Interface),
    ExternalItem(Box<External>)
}
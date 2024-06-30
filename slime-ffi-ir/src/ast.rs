#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Ident(String);

impl<T: ToString> From<T> for Ident {
    fn from(value: T) -> Self {
        Self(value.to_string())
    }
}

pub enum KeywordType {
    Int8,
    Int16,
    Int32,
    Int64,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Float32,
    Float64,
    Bool,
    String,
    Bytes,
}

pub enum RefType {
    QualifiedName(QualifiedName),
    Ident(Ident),
}

// Qualified Path
pub struct Pat(Vec<Ident>);

pub struct QualifiedName {
    pub path: Pat,
    pub name: Ident,
}

pub struct MapType {
    pub key: Type,
    pub value: Type,
}

pub enum BuiltinType {
    Optional(Box<Type>),
    List(Box<Type>),
    Map(Box<MapType>),
}

pub enum Type {
    Keyword(KeywordType),
    RefType(RefType),
    Builtin(BuiltinType),
}

pub struct FieldDecl {
    pub name: Ident,
    pub ty: Type,
}

pub struct ModelDecl {
    pub name: Ident,
    pub fields: Vec<FieldDecl>,
}

pub enum EnumVariantFields {
    Unit,
    Unnamed(Vec<Type>),
    Named(Vec<FieldDecl>),
}

pub struct EnumVariant {
    pub name: Ident,
}

pub struct EnumDecl {
    pub name: Ident,
    pub variants: Vec<EnumVariant>,
}

pub struct TypeParam {
    pub name: Ident,
    pub constraint: Option<Box<Type>>,
}

pub struct Function {
    pub params: Vec<FieldDecl>,
    pub type_params: Option<Vec<TypeParam>>,
    pub return_type: Option<Box<Type>>,
    pub error_type: Option<Box<Type>>,
}

pub struct FnDecl {
    pub name: Ident,
    pub function: Box<Function>,
}

pub struct TypeWithArgs {
    pub ty: Box<Ident>,
    pub type_args: Option<Vec<Box<Type>>>,
}

pub struct InterfaceDecl {
    pub name: Ident,
    pub type_params: Option<Vec<TypeParam>>,
    pub extends: Vec<TypeWithArgs>,
}

pub struct ConstDecl {
    pub name: Ident,
    pub ty: Type,
    pub value: Lit,
}

pub struct ConstEnumVariant {
    pub ty: Type,
    pub variant_name: Ident,
}

pub enum ConstLit {
    Primitive(Lit),
    UnitEnumVariant(ConstEnumVariant),
}

pub enum Expr {
    Ident(Ident),
    Lit(Lit),
}

pub enum Lit {
    Str(String),
    Num(NumLit),
    Bool(bool),
}

pub enum NumLit {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
}

pub struct CtorDecl {
    pub function: Box<Function>,
}

pub struct MethodDecl {
    pub name: Ident,
    pub function: Box<Function>,
}

pub enum ClassMember {
    Field(FieldDecl),
    Method(FnDecl),
    StaticMethod(FnDecl),
    Constructor(CtorDecl),
    Const(ConstDecl),
}

pub struct ClassDecl {
    pub name: Ident,
    pub extend: Option<Box<TypeWithArgs>>,
    pub impls: Option<Vec<Box<TypeWithArgs>>>,
    pub member: Vec<ClassMember>,
    pub type_params: Option<Vec<TypeParam>>,
}

pub struct ModDecl {
    pub name: Ident,
    pub items: Vec<Box<Decl>>,
}

pub enum Decl {
    Mod(ModDecl),
    Const(ConstDecl),
    Model(ModelDecl),
    Enum(EnumDecl),
    Method(FnDecl),
    Interface(InterfaceDecl),
    Class(ClassDecl),
}

pub struct Module {
    pub name: String,
    pub config: (),
    pub decls: Vec<Decl>,
}

#[doc(hidden)]
mod __impls {
    use std::ops::Deref;

    use super::{CtorDecl, FnDecl, Function};

    impl Deref for FnDecl {
        type Target = Function;

        fn deref(&self) -> &Self::Target {
            self.function.as_ref()
        }
    }

    impl Deref for CtorDecl {
        type Target = Function;

        fn deref(&self) -> &Self::Target {
            self.function.as_ref()
        }
    }
}

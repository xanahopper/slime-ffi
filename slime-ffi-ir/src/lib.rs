pub mod ast {
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

    pub struct Field {
        pub name: Ident,
        pub ty: Type,
    }

    pub struct ModelDecl {
        pub name: Ident,
        pub fields: Vec<Field>,
    }

    pub enum EnumVariant {
        Unit,
    }

    pub struct EnumDecl {

    }

    pub enum Decl {
        Model(ModelDecl)
    }
}
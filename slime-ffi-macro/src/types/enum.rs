use crate::types::{Field, Type};
use serde::{Deserialize, Serialize};

pub enum EnumVariantType {
    Primitive,
    Unnamed(Vec<Type>),
    Named(Vec<Field>),
}

pub struct EnumVariant {
    pub name: String,
    pub variant_type: EnumVariantType,
}

pub struct EnumItem {
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

impl EnumItem {
    pub fn is_primitive(&self) -> bool {
        self.variants
            .iter()
            .all(|v| matches!(v.variant_type, EnumVariantType::Primitive))
    }
}


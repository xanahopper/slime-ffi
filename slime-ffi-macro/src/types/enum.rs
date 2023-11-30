use serde::{Deserialize, Serialize};
use crate::types::{Field, Type};

#[derive(Debug, Serialize, Deserialize)]
pub enum EnumVariantType {
    Primitive,
    Unnamed(Vec<Type>),
    Named(Vec<Field>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,
    pub variant_type: EnumVariantType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnumItem {
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

impl EnumItem {
    pub fn is_primitive(&self) -> bool {
        self.variants.iter().all(|v| matches!(v.variant_type, EnumVariantType::Primitive))
    }
}
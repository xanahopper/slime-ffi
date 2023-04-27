use num_enum::IntoPrimitive;
use variant_count::VariantCount;
use slime_ffi_rt::runtime::Language;

pub trait Config {
    const LANGUAGE: Language;

    fn package() -> String;
}

pub struct Field {
    pub name: String,
    pub r#type: String,
}

pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>
}

pub struct Project {
    pub name: String,
    pub addon_types: Vec<Language>,
    pub structs: Vec<Struct>
}

pub mod scheme;

#[derive(VariantCount, IntoPrimitive)]
#[repr(usize)]
pub enum NameType {
    Function = 0,
    Method,
    Class,
    Interface,
    Parameter,
    Variable,
    Constant,
}

pub trait NameTransformer {
    fn transform(name: &str, r#type: &NameType) -> String;
}

pub struct TypedNameTransformer {
    transformers: [Option<Box<dyn NameTransformer>>; NameType::VARIANT_COUNT],
}

pub struct DefaultNameTransformer;

impl NameTransformer for DefaultNameTransformer {
    fn transform(name: &str, _type: &NameType) -> String {
        name.to_string()
    }
}

const DEFAULT_NAME_TRANSFORMER: DefaultNameTransformer = DefaultNameTransformer;

impl TypedNameTransformer {
    pub fn new() -> Self {
        Self {
            transformers: [None; NameType::VARIANT_COUNT],
        }
    }

    pub fn set(&mut self, r#type: NameType, transformer: impl NameTransformer + 'static) {
        self.transformers[r#type as usize] = Some(Box::new(transformer));
    }
}

impl NameTransformer for TypedNameTransformer {
    fn transform(name: &str, r#type: &NameType) -> String {
        let transformer: &Option<Box<dyn NameTransformer>> = &Self.transformers[r#type.into()];
        if let Some(transformer) = transformer.as_ref() {
            transformer.transform(name, r#type)
        } else {
            DEFAULT_NAME_TRANSFORMER.transform(name, r#type)
        }
    }
}
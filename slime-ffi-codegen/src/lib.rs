use std::error::Error;

use slime_ffi_ir::Lang;

pub enum CodegenError {
    Custom(Box<dyn Error>),
}

pub trait Codegen {
    const LANG: Lang;

    type Output;

    fn generate(&self) -> Result<Self::Output, CodegenError>;
}

pub mod lang;

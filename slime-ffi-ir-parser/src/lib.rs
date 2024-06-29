use std::path::Path;

use slime_ffi_ir::ast::Module;

pub enum ParseError {
    IO(std::io::Error),
}

pub trait Parser {
    fn parse(idl_path: &Path) -> Result<Module, ParseError>;
}

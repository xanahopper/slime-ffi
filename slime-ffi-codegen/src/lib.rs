use slime_ffi_ir::Lang;

pub trait Codegen {
    const LANG: Lang;
}

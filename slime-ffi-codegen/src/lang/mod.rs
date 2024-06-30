pub mod rust {
    use std::path::PathBuf;

    use dashmap::DashMap;
    use slime_ffi_ir::{ast::Ident, Lang};

    use crate::Codegen;

    pub struct RustCodegen {}

    #[derive(Debug)]
    pub struct RustPath(Vec<Ident>);

    #[derive(Debug)]
    pub struct RustType {
        pub pat: RustPath,
        pub name: Ident,
    }

    pub struct Context {
        pub ns: Vec<String>,
        pub type_locs: DashMap<RustType, RustPath>,
    }

    pub enum RustCodegenOutput {
        Source(PathBuf),
        Mod(PathBuf, Vec<Box<RustCodegenOutput>>),
    }

    impl Codegen for RustCodegen {
        const LANG: Lang = Lang::Rust;

        type Output = RustCodegenOutput;

        fn generate(&self) -> Result<Self::Output, crate::CodegenError> {
            todo!()
        }
    }
}

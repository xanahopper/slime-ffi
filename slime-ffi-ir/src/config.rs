pub enum LangConfig {
    Rust,
    Jvm(JvmConfig),
    Kotlin(KotlinConfig),
}

pub struct JvmConfig {
    pub package_name: String,
}

pub enum KotlinUnsignedStrategy {
    Signed,
    KotlinSigned,
}

pub struct KotlinConfig {
    pub unsigned_strategy: KotlinUnsignedStrategy,
}

pub struct SwiftConfig {
    pub module_name: String,
}

pub struct ObjcConfig {}

pub enum NodeDialect {
    TypeScript,
    JavaScriptWithDecl,
    ArkTS,
}

pub struct NodeConfig {
    pub dialect: NodeDialect,
}

pub enum WebDialet {
    TypeScript,
    JavaScriptWithDecl,
    WebAssambly,
}

pub struct WebConfig {
    pub dialect: WebDialet,
}

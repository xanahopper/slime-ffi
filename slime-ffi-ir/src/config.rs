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

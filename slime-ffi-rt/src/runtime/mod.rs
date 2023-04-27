use std::ffi::c_void;
use std::sync::Arc;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use jvm::JvmRuntime;
use crate::runtime::common::CommonRuntime;
use crate::runtime::dart::DartRuntime;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Language {
    C = 0,
    Jvm = 1,
    Dart = 2,
}

#[enum_delegate::register]
pub trait Runtime {
    fn initialize();
}

#[enum_delegate::implement(Runtime)]
pub enum FrontendRuntime {
    C(CommonRuntime),
    Jvm(JvmRuntime),
    DartRuntime(DartRuntime),
}

pub mod common;
#[cfg(feature = "jvm")]
pub mod jvm;
pub mod dart;

#[no_mangle]
pub unsafe extern "C" fn slime_create_runtime(
    language: u8,
    library_id: u64,
    dialect_data: * c_void,
) -> *const c_void {
    if let Ok(language) = Language::try_from(language) {
        let rt: Arc<FrontendRuntime> = Arc::new(match language {
            Language::C => common::create_common_runtime(library_id,dialect_data as _),
            #[cfg(feature = "jvm")]
            Language::Jvm => jvm::create_jvm_runtime(library_id, dialect_data as _),
            Language::Dart => dart::create_dart_runtime(library_id, dialect_data as _),
        });
        Arc::into_raw(rt) as _
    } else {
        std::ptr::null() as _
    }
}
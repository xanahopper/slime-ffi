use std::{ffi::c_void, str::FromStr};
use std::sync::Arc;
use num_enum::{IntoPrimitive, TryFromPrimitive};
#[cfg(feature = "jvm")]
use jvm::JvmRuntime;
use crate::runtime::common::CommonRuntime;

use self::common::CommonDialectData;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Language {
    C = 0,
    #[cfg(feature = "jvm")]
    Jvm = 1,
}

impl FromStr for Language {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "c" | "common" => Ok(Language::C),
            #[cfg(feature = "jvm")]
            "jvm" => Ok(Language::Jvm),
            _ => Err(()),
        }
    }
}
pub trait Runtime {
    fn initialize();
}
pub enum FrontendRuntime {
    C(CommonRuntime),
    #[cfg(feature = "jvm")]
    Jvm(JvmRuntime),
}

pub mod common;
#[cfg(feature = "jvm")]
pub mod jvm;

#[no_mangle]
pub unsafe extern "C" fn slime_create_runtime(
    language: u8,
    library_id: u64,
    dialect_data: *const c_void,
) -> *const c_void {
    if let Ok(language) = Language::try_from(language) {
        let rt: Arc<FrontendRuntime> = Arc::new(match language {
            Language::C => common::create_common_runtime(library_id, &*(dialect_data as *const CommonDialectData)),
            #[cfg(feature = "jvm")]
            _ => panic!(),
            // Language::Jvm => jvm::create_jvm_runtime(JavaVM::from_raw(&*(dialect_data as *const JvmInitializeData)).java_vm),
        });
        Arc::into_raw(rt) as _
    } else {
        std::ptr::null() as _
    }
}
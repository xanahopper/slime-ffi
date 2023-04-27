use std::ffi::c_void;
use std::sync::Arc;
use jni::{JavaVM, JNIEnv};
use jni::objects::JObject;
use jni::sys::{jint, jlong, JNI_VERSION_1_8};
use once_cell::sync::OnceCell;
use crate::runtime::{FrontendRuntime, Runtime};

const CALLBACK_NAME: &'static str = "com.slime.ffi.Callback";
static JAVA_VM: OnceCell<JavaVM> = OnceCell::new();

pub struct JvmRuntime {

}

impl Runtime for JvmRuntime {
    fn initialize() {
        todo!()
    }
}

#[repr(C)]
pub struct JvmInitializeData {

}

pub fn create_jvm_runtime(library_id: u64, dialect_data: &JvmInitializeData) -> FrontendRuntime {
    FrontendRuntime::Jvm(JvmRuntime {})
}

#[cfg(feature = "jvm_entry")]
#[allow(non_snake_case)]
pub fn JNI_OnLoad(vm: JavaVM, _reserved: c_void) -> jint {
    on_jvm_load(vm);
    JNI_VERSION_1_8
}

#[cfg(feature = "jvm")]
pub fn on_jvm_load(vm: JavaVM) {
    if JAVA_VM.set(vm).is_err() {
        eprintln!("Double initialization for JavaVM!");
    }
}
use std::any::TypeId;
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::{Arc, RwLock};
use jni::{JavaVM, JNIEnv, NativeMethod};
use jni::objects::{GlobalRef, JFieldID, JMethodID, JObject, JStaticFieldID, JStaticMethodID};
use jni::sys::{jint, jlong, JNI_VERSION_1_8};
use once_cell::sync::OnceCell;
use crate::runtime::{FrontendRuntime, Runtime};

type Error = jni::errors::Error;

const CALLBACK_NAME: &'static str = "com.slime.ffi.Callback";
static JAVA_VM: OnceCell<JavaVM> = OnceCell::new();

struct FieldMeta {
    pub field_id: JFieldID,
}

struct MethodMeta {
    pub method_id: JMethodID,
}

struct StaticFieldMeta {
    pub field_id: JStaticFieldID,
}

struct StaticMethodMeta {
    pub method_id: JStaticMethodID,
}

enum PropertyMeta {
    Field(FieldMeta),
    Method(MethodMeta),
    StaticField(StaticFieldMeta),
    StaticMethod(StaticMethodMeta),
}

struct JvmMeta {
    pub class: GlobalRef,
    pub properties: HashMap<&'static str, PropertyMeta>,
}

pub struct JvmNativeMeta {
    pub native_methods: Vec<NativeMethod>,
}

pub struct JvmObjectRegistry {
    pub native_meta: JvmNativeMeta,
    pub meta: JvmMeta,
}

pub trait JvmObject {
    type StaticType: 'static;

    fn register(env: &mut JNIEnv) -> Result<JvmNativeMeta, Error>;
}

pub struct JvmRuntime {
    pub java_vm: JavaVM,
    metas: RwLock<HashMap<TypeId, OnceCell<JvmMeta>>>,
}

impl Runtime for JvmRuntime {
    fn initialize() {

    }
}

pub struct JvmFieldDesc<'a> {
    pub name: &'static str,
    pub backend_name: &'static str,
    pub sig: &'a str,
}

#[macro_export]
macro_rules! backend_field {
    ($name: expr, $sig: expr) => {
        JvmFieldDesc {
            name: stringify!($name),
            backend_name: concat!("_", stringify!($name)),
            sig: $sig,
        }
    };
}

#[macro_export]
macro_rules! field {
    ($name: expr, $sig: expr) => {
        $crate::runtime::jvm::JvmFieldDesc {
            name: stringify!($name),
            backend_name: stringify!($name),
            sig: $sig,
        }
    };
}

#[repr(C)]
pub struct JvmInitializeData {

}

pub fn create_jvm_runtime(java_vm: jni::JavaVM) -> FrontendRuntime {
    FrontendRuntime::Jvm(JvmRuntime { java_vm, metas: RwLock::new(HashMap::new()) })
}

#[cfg(all(feature = "jvm", feature = "jvm_entry"))]
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
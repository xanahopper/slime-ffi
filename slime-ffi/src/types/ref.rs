use std::{any::TypeId, marker::PhantomData, ptr::NonNull};

struct VTable<RT, M, N> {
    pub(crate) runtime: RT,
    pub(crate) ctor: M,
    pub(crate) dtor: N,
    pub(crate) methods: Vec<(u32, NonNull<()>)>,
}

mod common {
    use std::ptr::NonNull;

    use super::VTable;

    pub type FfiAny = Option<NonNull<()>>;
    pub type CtorFn = unsafe fn(FfiAny) -> FfiAny;
    pub type DtorFn = unsafe fn(FfiAny);

    pub type FfiCVTable = VTable<(), CtorFn, DtorFn>;
}

pub use common::*;

#[cfg(feature = "jvm")]
mod jvm {
    use super::VTable;

    use crate::__private::jni::{objects::JMethodID, JavaVM};

    pub type JavaRuntimeVTable = VTable<JavaVM, JMethodID, JMethodID>;
}

#[cfg(feature = "jvm")]
pub use jvm::*;

enum FfiRefVTable {
    C(FfiCVTable),
    #[cfg(feature = "jvm")]
    Java(JavaRuntimeVTable),
}

impl Default for FfiRefVTable {
    fn default() -> Self {
        FfiRefVTable::C(FfiCVTable {
            runtime: (),
            ctor: None,
            dtor: None,
            methods: vec![],
        })
    }
}

#[repr(C)]
pub struct FfiRef {
    inner_ref: Option<NonNull<()>>,
    type_id: NonNull<TypeId>,
    vtable: NonNull<FfiRefVTable>,
}

pub trait SlimeType {}

fn ref_ty<T: 'static>() -> NonNull<TypeId> {
    NonNull::new(Box::into_raw(Box::new(TypeId::of::<T>()))).unwrap()
}

impl FfiRef {
    pub fn none<T: 'static>() -> FfiRef {
        unsafe {
            FfiRef {
                inner_ref: None,
                vtable: NonNull::new_unchecked(&EMPTY_VTABLE as *const _ as _),
                type_id: ref_ty::<T>(),
            }
        }
    }
}

pub struct Ref<T> {
    inner: FfiRef,
    _phantom: PhantomData<T>,
}

impl<T: 'static> Ref<T> {
    pub fn from_ffi(value: FfiRef) -> Result<Self, ()> {
        unsafe {
            if *value.type_id.as_ref() == TypeId::of::<T>() {
                Ok(Ref::<T> {
                    inner: value,
                    _phantom: PhantomData::default(),
                })
            } else {
                // TODO: implement a error type
                Err(())
            }
        }
    }
}

use std::{any::TypeId, marker::PhantomData, ptr::NonNull};

const EMPTY_VTABLE: FfiRefVTable = FfiRefVTable::C(FfiCVTable {
    ctor: None,
    dtor: None,
});

#[derive(Clone, Copy)]
struct FfiCVTable {
    pub(crate) ctor: Option<NonNull<unsafe fn(Option<NonNull<()>>) -> Option<NonNull<()>>>>,
    pub(crate) dtor: Option<NonNull<unsafe fn(Option<NonNull<()>>)>>,
}

struct FfiRuntimeVTable<RT, M> {
    pub(crate) runtime: RT,
    pub(crate) ctor: M,
    pub(crate) dtor: M,
}

enum FfiRefVTable {
    C(FfiCVTable),
    #[cfg(feature = "jvm")]
    Java(JavaRuntimeVTable),
}

#[cfg(feature = "jvm")]
type JavaRuntimeVTable =
    FfiRuntimeVTable<crate::__private::jni::JavaVM, crate::__private::jni::objects::JMethodID>;

#[repr(C)]
pub struct FfiRef {
    inner_ref: Option<NonNull<()>>,
    vtable: NonNull<FfiRefVTable>,
    type_id: NonNull<TypeId>,
}

pub trait SlimeType {}

unsafe fn ref_ty<T: 'static>() -> NonNull<TypeId> {
    NonNull::new_unchecked(Box::into_raw(Box::new(TypeId::of::<T>())))
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
                Err(())
            }
        }
    }
}

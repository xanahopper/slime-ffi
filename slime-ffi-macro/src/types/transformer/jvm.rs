use std::iter::once;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{PathSegment, TypePath};
use crate::types::{EnumItem, ModelItem, PrimitiveType, Type};

pub struct JvmTransformer {

}

impl JvmTransformer {
    pub fn generate_converter(ty: &Type) -> TokenStream {
        match ty {
            Type::Primitive(pri_type) => generate_primitive_converter(pri_type),
            Type::String => quote! {},
            Type::Bytes => quote! {},
            Type::Enum(item) => generate_enum_converter(item),
            Type::Model(item) => generate_model_converter(item),
            Type::Class(_) => todo!(),
            Type::Interface(_) => todo!(),
            Type::Option(_) => todo!(),
            Type::List(_) => todo!(),
            Type::Map { key, value } => todo!(),
            _ => panic!(),
        }
    }

    pub fn generate_definition(ty: &Type) -> TokenStream {
        todo!()
    }
}

fn generate_primitive_converter(primitive_type: &PrimitiveType) -> TokenStream {
    let jvm_ty = generate_primitive_type(primitive_type);
    let ty = super::common::generate_primitive_type(primitive_type);
    let result_smt = match primitive_type {
        PrimitiveType::Bool => quote! { value == jni::sys::JNI_TRUE },
        _ => quote! { value as #ty },
    };
    TokenStream::from(quote! {
        #[cfg(feature = "jvm")]
        impl slime_ffi::TryFromWith<#jvm_ty, &slime_ffi::runtime::jvm::JvmRuntime> for #ty {
            type Error = jni::errors::Error;

            fn try_from_with(value: #jvm_ty, _rt: &slime_ffi::runtime::jvm::JvmRuntime) -> core::result::Result<Self, Self::Error> {
                #result_smt
            }
        }
    })
}

fn generate_enum_converter(enum_type: &EnumItem) -> TokenStream {
    let jvm_ty = generate_enum_type(enum_type);
    let ty: syn::Type = syn::parse_str(&enum_type.name).unwrap();
    let smt = if enum_type.is_primitive() {
        quote! { <#ty>::try_from(value).map_err(|e| jni::errors::Error::JavaException) }
    } else {
        todo!()
    };
    TokenStream::from(quote! {
        #[cfg(feature = "jvm")]
        impl<'a> slime_ffi::TryFromWith<#jvm_ty, &'a slime_ffi::runtime::jvm::JvmRuntime> for #ty {
            type Error = jni::errors::Error;

            fn try_from_with(value: #jvm_ty, rt: &'a slime_ffi::runtime::jvm::JvmRuntime) -> core::result::Result<Self, jni::errors::Error> {
                #smt
            }
        }
    })
}

fn generate_model_converter(item: &ModelItem) -> TokenStream {
    let jvm_ty = generate_model_type(item);
    let ty: syn::Type = syn::parse_str(&item.name).unwrap();
    TokenStream::from(quote! {
        #[cfg(feature = "jvm")]
        impl<'a> slime_ffi::TryFromWith<#jvm_ty, &'a slime_ffi::runtime::jvm::JvmRuntime> for #ty {
            type Error = jni::errors::Error;

            fn try_from_with(value: #jvm_ty, rt: &'a slime_ffi::runtime::jvm::JvmRuntime) -> core::result::Result<Self, Self::Error> {
                todo!()
            }
        }
    })
}

fn generate_model_definition(item: &ModelItem) -> TokenStream {
    let ty: syn::Type = syn::parse_str(item.name.as_str()).unwrap();

    TokenStream::from(quote! {
        impl slime_ffi::runtime::jvm::JvmObject for #ty {
            fn register(env: &mut jni::JNIEnv) -> Result<slime_ffi::runtime::jvm::JvmObjectRegistry, slime_ffi::runtime::jvm::Error> {
                // 1. get class
                // 2. get properties
                // 2.1 get ctor
                // 2.2 get fields
                // 2.3 get methods
                // 2.4 get static fields and methods
                // 3. generate native method
            }
        }
    })
}

fn generate_primitive_type(ty: &PrimitiveType) -> syn::Type {
    let name = match ty {
        PrimitiveType::Int8 => "i8",
        PrimitiveType::Int16 => "i16",
        PrimitiveType::Int32 => "i32",
        PrimitiveType::Int64 => "i64",
        PrimitiveType::UInt8 => "u8",
        PrimitiveType::UInt16 => "u16",
        PrimitiveType::UInt32 => "u32",
        PrimitiveType::UInt64 => "u64",
        PrimitiveType::Bool => "bool",
        PrimitiveType::Float32 => "f32",
        PrimitiveType::Float64 => "f64",
    };
    let ident = Ident::new(name, Span::call_site());
    let segment = PathSegment { ident, arguments: syn::PathArguments::None };
    syn::Type::Path(TypePath { qself: None, path: syn::Path { leading_colon: None, segments: once(segment).collect() } })
}

fn generate_enum_type(enum_type: &EnumItem) -> syn::Type {
    if enum_type.is_primitive() {
        syn::parse_quote!(jni::sys::jint)
    } else {
        syn::parse_quote!(jni::objects::JObject<'a>)
    }
}

fn generate_model_type(item: &ModelItem) -> syn::Type {
    syn::parse_quote!(jni::Objects::JObject<'a>)
}


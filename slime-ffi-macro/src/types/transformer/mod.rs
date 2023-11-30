use std::iter::once;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, TokenStreamExt, ToTokens};
use syn::{PathSegment, TypePath};
use crate::types::{ModelItem, PrimitiveType, Type};

pub mod common;
pub mod jvm;


impl ToTokens for PrimitiveType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty = match self {
            PrimitiveType::Int8 => quote!(i8),
            PrimitiveType::Int16 => quote!(i16),
            PrimitiveType::Int32 => quote!(i32),
            PrimitiveType::Int64 => quote!(i64),
            PrimitiveType::UInt8 => quote!(u8),
            PrimitiveType::UInt16 => quote!(u16),
            PrimitiveType::UInt32 => quote!(u32),
            PrimitiveType::UInt64 => quote!(u64),
            PrimitiveType::Bool => quote!(bool),
            PrimitiveType::Float32 => quote!(f32),
            PrimitiveType::Float64 => quote!(f64),
        };
    }
}

impl From<&PrimitiveType> for syn::Type {
    fn from(value: &PrimitiveType) -> Self {
        let name = match value {
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
}

impl ToTokens for ModelItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let model_name = Ident::new(self.name.as_str(), Span::mixed_site());
        let c_name = Ident::new(&format!("_slm_{}", model_name), Span::mixed_site());
        let fields: Vec<(Ident, Type)> = self.fields
            .iter()
            .map(|f| {
                (
                    Ident::new(&f.name, Span::call_site()),
                    f.r#type.into(),
                )
            })
            .collect();
        let c_fields: Vec<(Ident, Type)> = self.fields
            .iter()
            .map(|f| {
                (
                    Ident::new(&f.name, Span::call_site()),
                    // FIXME: replace type into c represent
                    f.r#type.into(),
                )
            })
            .collect();
        tokens.append(quote! {
            pub struct #model_name {
                #(pub #fields.0: #fields.1),*
            }

            #[cfg(feature = "jvm")]
            impl<'a> slime_ffi_rt::TryFromWith<jni::objects::JObject<'a>, &'a slime_ffi_rt::runtime::jvm::JvmRuntime> for #model_name {
                type Error = jni::errors::Error;

                fn try_from_with(value: JObject<'a>, rt: &'a slime_ffi_rt::runtime::jvm::JvmRuntime) -> Result<Self, Self::Error> {
                    todo!()
                }
            }

            #[cfg(feature = "jvm")]
            impl<'a> slime_ffi_rt::TryFromWith<#model_name, &'a slime_ffi_rt::runtime::jvm::JvmRuntime> for jni::objects::JObject<'a> {
                type Error = jni::errors::Error;

                fn try_from_with(value: #model_name, rt: &'a slime_ffi_rt::runtime::jvm::JvmRuntime) -> Result<Self, Self::Error> {
                    todo!()
                }
            }

            #[cfg(feature = "common")]
            #[repr(C)]
            #[allow(non_snake_case)]
            pub struct #c_name {
                #(pub #c_fields.0: #c_fields.1),*
            }

            #[cfg(feature = "common")]
            impl slime_ffi_rt::TryFromWith<#c_name, &slime_ffi_rt::runtime::common::CommonRuntime> for #model_name {
                type Error = ();

                fn try_from_with(value: #c_name, rt: &slime_ffi_rt::runtime::common::CommonRuntime) -> Result<Self, ()> {
                    todo!()
                }
            }
        });
    }
}
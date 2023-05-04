use proc_macro::TokenTree;
use proc_macro2::TokenStream;
use syn::{bracketed, Error, Item, ItemMod, parse_macro_input, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::visit::Visit;


enum InterfaceType {
    C,
    Java,
    Kotlin,
    ObjectiveC,
    Swift,
    Dart,
    Python,
    Napi,
}

struct PackageInfo {
    name: String,
    package: String,
    interface: Vec<InterfaceType>
}

pub(crate) struct SlimeFfiModule {
    pub(crate) module_decl: ItemMod,
    pub(crate) package_info: PackageInfo,
}

impl Parse for SlimeFfiModule {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let module_decl: ItemMod = input.parse().map_err(|e| {
            Error::new(
                e.span(),
                "`slime_ffi_package` attribute is supported on mod items only",
            )
        })?;
    }
}

pub(crate) struct ModuleVisitor {

}

impl<'ast> Visit<'ast> for ModuleVisitor {
    fn visit_item(&mut self, i: &'ast Item) {

    }
}

pub mod transform {
    use proc_macro2::TokenStream;
    use syn::fold::Fold;
    use syn::{Item, ItemImpl, ItemStruct, ItemTrait};
    use crate::SlimeFfiModule;

    pub enum NameType {
        Variable,
        Type,
        Method,
    }

    pub trait NameTransformer {
        fn transform_name(name_type: NameType, name: &str) -> String;
    }

    pub(crate) struct ModTransformer {
        module: SlimeFfiModule,
    }

    impl ModTransformer {
        pub fn new(module: SlimeFfiModule) -> Self {
            Self { module }
        }

        // class - methods
        pub(crate) fn transform_item_impl(&mut self, node: ItemImpl) -> TokenStream {
            todo!()
        }
    }

    impl Fold for ModTransformer {
        fn fold_item(&mut self, node: Item) -> Item {
            match node {
                Item::Impl(i) => Item::Verbatim(self.transform_item_impl(i)),
                Item::Trait(t) => Item::Trait(self.fold_item_trait(t)),
                Item::Struct(s) => Item::Struct(self.fold_item_struct(s)),
                _ => node,
            }
        }

        // class
        fn fold_item_struct(&mut self, node: ItemStruct) -> ItemStruct {
            todo!()
        }

        // interface
        fn fold_item_trait(&mut self, i: ItemTrait) -> ItemTrait {
            todo!()
        }
    }
}

///
///
/// # Arguments
///
/// * `attr`:
/// * `item`:
///
/// returns: TokenStream
///
/// # Examples
///
/// ```rust
/// #[ffi_package {
///     name = "ClientFfi",
///     package = "com.github.slime.ffi",
///     interfaces = ["C", "Java", "Kotlin", "ObjC", "Swift", "Dart", "Python", "Napi"],
///     name_transformer = {
///         java = CustomJavaTransformer
///     },
///
/// }]
/// pub mod ffi {
///     use std::collections::HashMap;
///     #[rename(java = "acc", dart = "add_int64")]
///     #[exclude(kotlin, python, napi)]
///     pub fn add(left: i64, right: i64) -> i64 {
///         left + right
///     }
///
///     #[copy]
///     pub struct Config {
///     }
///
///     #[rename(java = "Request")]
///     pub struct Request {
///         pub url: String,
///         pub headers: Option<HashMap<String, String>>,
///         pub body: Vec<u8>,
///     }
///
///     pub struct Response {
///         pub code: u32,
///         pub content: Vec<u8>
///     }
///
///     pub struct Client {
///         agent: String,
///         version: i32,
///         token: u64,
///     }
///
///     impl Client {
///         #[constructor]
///         pub fn new(agent: String, version: i32, token: u64) -> Self {
///             Client {
///                 agent, version, token,
///             }
///         }
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn ffi_package(attr: TokenStream, item: TokenStream) -> TokenStream {
    let package_info = parse_macro_input!(attr as AttributeArgs);
    item
}
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{visit::Visit, ItemMod, Ident, Type, ItemFn, Visibility, Token, Signature, ReturnType, TypePath, Path, PathSegment, Block};
use slime_ffi_gen::Language;
use crate::types::Item;

mod types;
mod scheme;

#[proc_macro_attribute]
pub fn slime(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(quote! {})
}

struct Transformer(Item, Language);

struct ModCollector {
    pub items: Vec<Item>,
}

impl<'ast> Visit<'ast> for ModCollector {
    fn visit_attr_style(&mut self, i: &'ast syn::AttrStyle) {}

    fn visit_attribute(&mut self, i: &'ast syn::Attribute) {}

    fn visit_data_struct(&mut self, i: &'ast syn::DataStruct) {}

    fn visit_item_mod(&mut self, item: &'ast ItemMod) {
        let ItemMod { attrs, vis, unsafety, mod_token, ident, content, semi } = item;
        // if let Some((_, items)) = content {
        //     self.items.push(Item::ModItem(Box::new(ModItem {
        //         name: ident.to_string(),
        //         items: items.iter().map(|item| self.visit_item(item)).collect(),
        //     })));
        // }
    }
}

impl ToTokens for Transformer {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self.0 {
            Item::FnItem(item) => todo!(),
            Item::ModelItem(_) => todo!(),
            Item::ClassItem(_) => todo!(),
            Item::InterfaceItem(_) => todo!(),
            Item::DependencyItem(_) => todo!(),
            Item::ExternalItem(_) => todo!(),
            Item::UseItem(_) => todo!(),
            Item::ModItem(_) => todo!(),
            Item::ImplItem(_) => todo!(),
        }
    }
}

impl Transformer {
    fn generate_fn_item(item: &types::FnItem) -> proc_macro2::TokenStream {
        let fn_name = Ident::new(item.name.as_str(), Span::mixed_site());
        let fun = ItemFn {
            attrs: vec![],
            vis: Visibility::Inherited,
            sig: Signature {
                constness: None,
                asyncness: None,
                unsafety: None,
                abi: None,
                fn_token: Default::default(),
                ident: fn_name,
                generics: Default::default(),
                paren_token: Default::default(),
                inputs: Default::default(),
                variadic: None,
                output: ReturnType::Type(Token![->](Span::mixed_site()), Box::new(Transformer::generate_type(&item.return_type))),
            },
            block: Box::new(Block { brace_token: Default::default(), stmts: vec![] }),
        };
        todo!()
    }

    fn generate_type(ty: &types::Type) -> syn::Type {
        match ty {
            types::Type::Void => todo!(),
            types::Type::Primitive(_) => todo!(),
            types::Type::String => todo!(),
            types::Type::Bytes => todo!(),
            types::Type::Enum(_) => todo!(),
            types::Type::Model(_) => todo!(),
            types::Type::Class(_) => todo!(),
            types::Type::Interface(_) => todo!(),
            types::Type::Option(_) => todo!(),
            types::Type::List(_) => todo!(),
            types::Type::Map { key, value } => todo!(),
        }
    }
}
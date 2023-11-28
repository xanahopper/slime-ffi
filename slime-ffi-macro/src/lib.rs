use quote::quote;
use syn::{visit::Visit, ItemMod};

#[proc_macro_attribute]
pub fn slime(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    
    proc_macro::TokenStream::from(quote! {})
}

struct ModCollector {
}

impl<'ast> Visit<'ast> for ModCollector {
    fn visit_item_mod(&mut self, item: &'ast ItemMod) {

    }
}
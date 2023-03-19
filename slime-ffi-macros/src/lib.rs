use proc_macro2::TokenStream;

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
/// #[slime_ffi_package {
///     name = "SlimeFfi",
///     package = "com.github.slime.ffi",
///     interfaces = ["C", "Java", "Kotlin", "Dart", "Python", "Napi"],
/// }]
/// pub mod ffi {
///     pub fn add(left: i64, right: i64) -> i64 {
///         left + right
///     }
///
///     pub struct BuiltinModel {
///         
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn slime_ffi_package(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
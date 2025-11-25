use proc_macro::TokenStream;

mod node;

#[proc_macro_attribute]
pub fn with_metadata(attr: TokenStream, item: TokenStream) -> TokenStream {
    node::metadata::expand_with_metadata(attr, item)
}

#[proc_macro_attribute]
pub fn with_parameter(attr: TokenStream, item: TokenStream) -> TokenStream {
    node::metadata::expand_with_parameter(attr, item)
}

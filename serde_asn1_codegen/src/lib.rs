extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro]
pub fn from_dir(input: TokenStream) -> TokenStream {
    let gen = quote! {
        #[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq)]
        struct Point {
            x: i64,
            y: i64,
        }
    };
    gen.into()
}

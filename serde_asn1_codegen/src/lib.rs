extern crate proc_macro;

use crate::proc_macro::{TokenStream, TokenTree};
use quote::quote;

fn parse_input(input: TokenStream) -> String {
    for token in input {
        if let TokenTree::Literal(s) = token {
            return format!("{}", s);
        }
        panic!("need to pass a string literal");
    }
    panic!("need to pass a string literal");
}

#[proc_macro]
pub fn from(input: TokenStream) -> TokenStream {
    let crate_root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut path = std::path::PathBuf::from(crate_root);
    let input_path_string = parse_input(input);
    let i = input_path_string.trim_start_matches('"').trim_end_matches('"');
    let input_path = std::path::PathBuf::from(&i);
    path.push(&input_path);

    if !path.exists() {
        panic!("Must provide a valid path");
    }
    let gen = quote! {
        #[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq)]
        struct Point {
            x: i64,
            y: i64,
        }
    };
    gen.into()
}

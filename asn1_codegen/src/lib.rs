extern crate proc_macro;

use crate::proc_macro::{TokenStream, TokenTree};
use proc_macro2::{Ident, Span};
use quote::quote;

mod asn_parser;
use asn_parser::{AsnModule, AsnType};

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
    let i = input_path_string
        .trim_start_matches('"')
        .trim_end_matches('"');
    let input_path = std::path::PathBuf::from(&i);
    path.push(&input_path);

    if !path.is_file() {
        panic!("Must provide path to a file");
    }
    let asn1_string = std::fs::read_to_string(path).unwrap();
    let asn_module = AsnModule::from(&*asn1_string);

    let mut out = TokenStream::new();

    for (struct_name, sequence) in asn_module.sequences {
        let struct_name = Ident::new(struct_name, Span::call_site());
        let fields = sequence
            .fields
            .iter()
            .map(|field| {
                let field_type = match field.field_type {
                    AsnType::Integer => "i64",
                    AsnType::Custom(t) => t,
                };
                let name = Ident::new(field.name, Span::call_site());
                let field_type = Ident::new(field_type, Span::call_site());
                quote! {
                    #name : #field_type ,
                }
            });
        // TODO consider field type to set appropriate type on Rust struct
        let gen: TokenStream = quote! {
            #[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, PartialEq)]
            struct #struct_name {
                #(#fields)*
            }
        }
        .into();
        out.extend(gen);
    }

    out
}

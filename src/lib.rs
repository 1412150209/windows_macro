use proc_macro::TokenStream;

use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(FromInto)]
pub fn my_derive(_input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(_input as DeriveInput);
    let _self = &ast.ident;
    let _other = match ast.data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Unnamed(ref fields) => {
                    if fields.unnamed.is_empty() {
                        panic!("There is no unnamed parameter in struct")
                    }
                    if fields.unnamed.iter().count().ne(&1) {
                        panic!("There are more than one unnamed parameter in struct")
                    } else {
                        fields.unnamed.first().unwrap()
                    }
                }
                _ => panic!("There is no unnamed parameter in struct")
            }
        }
        _ => panic!("This is not a struct")
    };
    let gen = quote! {
        impl From<#_other> for #_self{
            fn from(value: #_other) -> Self{
                Self(value)
            }
        }
        impl Into<#_other> for #_self{
            fn into(self) -> #_other{
                Self.0
            }
        }
    };
    gen.into()
}

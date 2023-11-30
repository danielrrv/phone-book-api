use proc_macro::{Ident, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::token::Struct;
use syn::{parse_macro_input, Data, DeriveInput, Result};
use mongodb::{
    bson::{doc, Document},
    Collection,
};

// #[proc_macro_attribute]
// pub fn collection(_: TokenStream, item: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(item as DeriveInput);
//     let struct_name = input.ident;
//     let (_, fields) = match input.data {
//         Data::Struct(data_struct) => (Some(data_struct.struct_token), Some(data_struct.fields)),
//         _ => (None, None),
//     };
//     let fields_names: Vec<_> = fields
//         .unwrap()
//         .iter()
//         .map(|field| quote! {#field})
//         .collect();

//     let expanded = quote! {
//         pub struct #struct_name {
//             // #[serde(skip)]
//             _collection:String,
//             #(#fields_names,)*
//         }
//     };

//     TokenStream::from(expanded)
// }

#[proc_macro_derive(Model)]
pub fn derive_collection(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = input.ident;
    let (_, fields) = match input.data {
        Data::Struct(data_struct) => (Some(data_struct.struct_token), Some(data_struct.fields)),
        _ => (None, None),
    };
    let fields_names: Vec<_> = fields
        .unwrap()
        .iter()
        .map(|field| quote! {#field})
        .collect();

    let expanded = quote! {

        impl Model for #struct_name {
            fn save<T>(&self, coll: Collection)->bool{
                coll.insert_one(self);
                println!("Hello from save!");
                true
            }

        }
    };
    TokenStream::from(expanded)
}
// #[cfg(test)]
// mod tests{
//     use crate::collection;
//     #[test]
//     fn first_test(){
//         assert_eq!(1, 1)
//     }
//     #[collection("businesses")]
//     struct Home{
//         owner:String
//     }
//     // fn collection_test(){

//     // }
// }

// model!(struct Home {
//     pub name:String,
// });

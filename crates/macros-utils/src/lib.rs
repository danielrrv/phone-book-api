use mongodb;
use proc_macro::{Ident, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::token::Struct;
use syn::{parse_macro_input, Data, DeriveInput, Result};



#[proc_macro_attribute]
pub fn collection(_: TokenStream, item: TokenStream) -> TokenStream {
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
        pub struct #struct_name {
            // #[serde(skip)]
            _collection:String,
            #(#fields_names,)*
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Collection)]
pub fn derive_collection(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = input.ident;
    let (_, fields) = match input.data {
        Data::Struct(data_struct) => (Some(data_struct.struct_token), Some(data_struct.fields)),
        _ => (None, None),
    };
    // let fields_names: Vec<_> = fields
    //     .unwrap()
    //     .iter()
    //     .map(|field| quote! {#field})
    //     .collect();

    let expanded = quote! {
        impl #struct_name {
            // pub fn from_collection(&mut self, entity: String) -> &mut Self {
            //     self._query.clone().unwrap().set_entity(entity.to_string());
            //     self
            // }
            fn find(&mut self, value: String)-> &mut T{
                    
            }
            // #[serde(skip)]
            // _collection:String,p
            // #(#fields_names,)*
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

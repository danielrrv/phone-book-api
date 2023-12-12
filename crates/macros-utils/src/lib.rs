use mongodb::{
    bson::{doc, Document},
    Collection,
};
use proc_macro::{Ident, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::token::Struct;
use syn::{parse_macro_input, Data, DeriveInput, Result};

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
    let DeriveInput {
        ident,
        attrs: _,
        vis: _,
        generics: _,
        data,
    } = parse_macro_input!(item as DeriveInput);
    let struct_name = ident;
    let (_, fields) = match data {
        Data::Struct(data_struct) => (Some(data_struct.struct_token), Some(data_struct.fields)),
        _ => (None, None),
    };
    let _fields_names: Vec<_> = fields
        .unwrap()
        .iter()
        .map(|field| quote! {#field})
        .collect();

    let expanded = quote! {
        impl Model for #struct_name {
            fn get_id(&mut self)-> &String{
                return self._id.as_ref().unwrap();
            }
            fn tag_from(&mut self, paratext: &str) -> &mut Self {
                let rregex: Regex = Regex::new(r"\p{L}{4, 16}").unwrap();
                // let mut tags: Vec<String> =  paratext.split)
                let mut tags: Vec<String> = rregex
                    .find_iter(&*paratext)
                    .map(|value| value.as_str())
                    .map(|value| value.to_string())
                    .collect();
                let mut filtered_tags: Vec<String> = self
                    .tags
                    .iter()
                    .filter(|tag| !tags.contains(tag))
                    .map(|tag| tag.to_string())
                    .collect();
                if filtered_tags.is_empty() {
                    self.tags.append(&mut tags);
                } else {
                    self.tags.append(&mut filtered_tags);
                    self.tags.dedup();
                }
                self
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

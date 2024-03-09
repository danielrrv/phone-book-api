use proc_macro::{TokenStream};
mod collection_derive;
use futures::stream::{StreamExt, TryStream, TryStreamExt};
use collection_derive::handle_collection_derive;
use syn::DeriveInput;

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

#[proc_macro_derive(Model, attributes(types_associated))]
pub fn derive_collection(item: TokenStream) -> TokenStream {
    handle_collection_derive(item)
}

#[cfg(test)]
mod tests{
    
    #[test]
    fn first_test(){
        assert_eq!(1, 1)
    }
 
    #[derive(Debug, Deserialize, Serialize, Clone, Model)]
    pub struct Business {
        pub _id: Option<String>,
        pub company_name: String,
        pub locations: Vec<String>,
        pub description: String,
        pub tags: Vec<String>,
    }
}
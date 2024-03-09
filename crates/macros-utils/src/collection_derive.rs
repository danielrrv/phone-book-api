use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Data, Attribute, token::Token, Generics};
use quote::quote;
use std::pin::Pin;
use futures::stream::{StreamExt, TryStream, TryStreamExt};
use serde::de::DeserializeOwned;
use std::default::Default;
use mongodb::{
    bson::{doc, Document},
    error::Error,
    options::FindOptions,
    Collection,
    
};


pub fn handle_collection_derive(item: TokenStream)-> TokenStream{
    let DeriveInput {
        ident,
        attrs:_,
        vis: _,
        generics ,
        data,
    } = parse_macro_input!(item as DeriveInput);

    let struct_name = ident;
    let (_, fields) = match data {
        Data::Struct(data_struct) => (Some(data_struct.struct_token), Some(data_struct.fields)),
        _ => (None, None),
    };
    println!("{:?}", generics);
    let _fields_names: Vec<_> = fields
        .unwrap()
        .iter()
        .map(|field| quote! {#field})
        .collect();

    let expanded = quote! {

     
        #[async_trait]
        impl Model for #struct_name  {
            fn get_id(&mut self)-> &String{
                return self._id.as_ref().unwrap();
            }
            fn tag_from(&mut self, paratext: &str) -> &mut Self {
                let rregex: Regex = Regex::new(r"\p{L}{4, 16}").unwrap();
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
            
            async fn find<#struct_name>(conn: &Collection<#struct_name>, filter: Document)-> Pin<Box<Result<Vec<#struct_name>, Error>>>
                where 
                #struct_name: Neweable<#struct_name> + DeserializeOwned,
                {
                let opts = FindOptions::builder().batch_size(50).build();
                let cursor = conn.find(filter, opts).await.unwrap();
                let mut container: Vec<#struct_name> = Vec::with_capacity(1000);
                while let Some(result) = cursor.next().await {
                    match result {
                        Ok(value) => container.push(value),
                        Err(error) => panic!("{}", error),
                    }
                }
                if container.is_empty(){
                    container.push(#struct_name::new())
                }
                Ok(container)
            }
        }
    };
    TokenStream::from(expanded)
}



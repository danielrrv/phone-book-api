
use async_trait::async_trait;
use futures::stream::{StreamExt, TryStream, TryStreamExt};



// macro_rules! model {
//     (
//      $(#[$meta:meta])*
//      struct $struct_name:ident {
//         $(
//         $(#[$field_meta:meta])*
//         $field_vis:vis $field_name:ident : $field_type:ty
//         ),*$(,)+
//     }
//     ) => {

//             $(#[$meta])*
//             pub struct $struct_name{
//                 $(
//                 $(#[$field_meta:meta])*
//                 pub $field_name : $field_type,
//                 )*
//             }

//             impl $struct_name {
//                 pub(crate) fn new($($field_name : $field_type)*) -> Self {
//                     Self { $($field_name)* }
//                 }
//             }
//     }
// }
// #[proc_macro_derive(Collection, attributes(serde))]
// pub fn derive_serialize(input: TokenStream) -> TokenStream {
//     let mut input = parse_macro_input!(input as DeriveInput);
//     ser::expand_derive_serialize(&mut input)
//         .unwrap_or_else(syn::Error::into_compile_error)
//         .into()
// }

// #[async_trait]
// pub macro Collection<T> {
//     fn find(&mut self, value: String)-> &mut T;
//     fn when(&mut self, field_name: String, field_value: String) -> &mut T;
//     fn from_collection(&mut self, entity: String) -> &mut T;
//     async fn execute(&mut self) -> Result<&T, mongodb::error::Error>;
// }



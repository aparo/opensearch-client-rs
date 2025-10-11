mod models;
mod opensearch_record;

use darling::FromDeriveInput;
use models::OpenSearchDeriveInput;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

#[proc_macro_derive(OpenSearch, attributes(os))]
pub fn derive_opensearch_record(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as syn::DeriveInput);
  let data = OpenSearchDeriveInput::from_derive_input(&input);
  let stream = match data {
    Ok(data) => data.into_token_stream(),
    Err(err) => err.write_errors(),
  };
  stream.into()
}

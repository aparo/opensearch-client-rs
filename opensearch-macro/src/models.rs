use darling::{FromDeriveInput, FromField, ast, util};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(os), supports(struct_named), forward_attrs(allow, doc, cfg))]
pub(crate) struct OpenSearchDeriveInput {
  pub(crate) ident: syn::Ident,
  pub(crate) data: ast::Data<util::Ignored, OpenSearchField>,
  pub(crate) generics: syn::Generics,
  pub(crate) index: String,
}

#[derive(Debug, FromField)]
#[darling(attributes(os))]
pub(crate) struct OpenSearchField {
  pub(crate) ident: Option<syn::Ident>,
  pub(crate) ty: syn::Type,

  #[darling(default)]
  pub(crate) skip: bool,

  #[darling(default)]
  pub(crate) id: bool,

  #[darling(default)]
  pub(crate) skip_sort: bool,

  #[darling(default)]
  pub(crate) none_value: Option<String>,
}

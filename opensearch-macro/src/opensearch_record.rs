use quote::{quote, ToTokens};
use syn::__private::TokenStream2;

use crate::models::OpenSearchDeriveInput;

/// Helper function to determine the OpenSearch field type from Rust type
fn get_opensearch_type(ty: &syn::Type) -> (String, String, bool, bool) {
    match ty {
        syn::Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                let type_name = segment.ident.to_string();
                match type_name.as_str() {
                    "String" | "str" => ("text".to_string(), "string".to_string(), false, true),
                    "i32" | "i64" | "u32" | "u64" | "isize" | "usize" => {
                        ("long".to_string(), "number".to_string(), true, true)
                    }
                    "f32" | "f64" => ("double".to_string(), "number".to_string(), true, true),
                    "bool" => ("boolean".to_string(), "boolean".to_string(), true, true),
                    "DateTime" => ("date".to_string(), "date".to_string(), true, true),
                    "Vec" => {
                        // For Vec<T>, extract the inner type
                        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first()
                            {
                                let (os_type, field_type, _, _) = get_opensearch_type(inner_type);
                                return (os_type, field_type, true, true);
                            }
                        }
                        ("text".to_string(), "string".to_string(), false, true)
                    }
                    "Option" => {
                        // For Option<T>, extract the inner type
                        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first()
                            {
                                return get_opensearch_type(inner_type);
                            }
                        }
                        ("text".to_string(), "string".to_string(), false, true)
                    }
                    _ => {
                        // For custom types, assume they implement Document trait
                        ("object".to_string(), "object".to_string(), false, false)
                    }
                }
            } else {
                ("text".to_string(), "string".to_string(), false, true)
            }
        }
        _ => ("text".to_string(), "string".to_string(), false, true),
    }
}

/// Check if a type is likely a custom struct that implements Document
fn is_custom_document_type(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                let type_name = segment.ident.to_string();
                match type_name.as_str() {
                    "String" | "str" | "i32" | "i64" | "u32" | "u64" | "isize" | "usize"
                    | "f32" | "f64" | "bool" | "DateTime" => false,
                    "Vec" => {
                        // For Vec<T>, check if T is a custom type
                        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first()
                            {
                                return is_custom_document_type(inner_type);
                            }
                        }
                        false
                    }
                    "Option" => {
                        // For Option<T>, check if T is a custom type
                        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first()
                            {
                                return is_custom_document_type(inner_type);
                            }
                        }
                        false
                    }
                    _ => true, // Assume it's a custom type
                }
            } else {
                false
            }
        }
        _ => false,
    }
}

impl ToTokens for OpenSearchDeriveInput {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let OpenSearchDeriveInput {
            ref ident,
            ref data,
            ref generics,
            index: ref index_name,
        } = *self;

        let fields = data.as_ref().take_struct().expect("Is not enum").fields;

        let generic_params = &generics.params;
        let where_clause = &generics.where_clause;
        let generic_params_wb = if generic_params.is_empty() {
            quote! {}
        } else {
            quote! {<#generic_params>}
        };

        let mut column_names = vec![];
        let mut columns = vec![];
        let mut document_fields = vec![];
        let mut id_field: Option<&syn::Ident> = None;

        // Process fields to generate Document trait fields
        for f in &fields {
            let name = f.ident.as_ref().expect("named field");
            let name_str = name.to_string();

            if f.skip {
                continue;
            }

            // Check if this is the ID field
            if f.id {
                id_field = Some(name);
            }

            let (os_type, field_type, aggregatable, searchable) = get_opensearch_type(&f.ty);

            columns.push(name);
            column_names.push(name_str.clone());

            // Check if this field has nested fields (custom Document type)
            if is_custom_document_type(&f.ty) {
                // Generate code to get sub-fields from the nested Document type
                let nested_type = &f.ty;
                document_fields.push(quote! {
                    opensearch_client::Field {
                        name: #name_str.to_string(),
                        field_type: #field_type.to_string(),
                        os_type: #os_type.to_string(),
                        aggregatable: #aggregatable,
                        searchable: #searchable,
                        sub_fields: {
                            // Get sub-fields from nested Document type
                            let nested_fields = <#nested_type as opensearch_client::Document>::columns();
                            nested_fields.into_iter().map(|f| Box::new(f)).collect()
                        },
                    }
                });
            } else {
                // Regular field without sub-fields
                document_fields.push(quote! {
                    opensearch_client::Field {
                        name: #name_str.to_string(),
                        field_type: #field_type.to_string(),
                        os_type: #os_type.to_string(),
                        aggregatable: #aggregatable,
                        searchable: #searchable,
                        sub_fields: vec![],
                    }
                });
            }
        }

        // Generate the id() method implementation
        let id_method = if let Some(id_field) = id_field {
            quote! {
                fn id(&self) -> &str {
                    &self.#id_field
                }
            }
        } else {
            // If no field is marked with #[os(id)], try to find a field named "id"
            let id_field_name = fields
                .iter()
                .find(|f| f.ident.as_ref().map(|i| i.to_string()) == Some("id".to_string()))
                .and_then(|f| f.ident.as_ref());

            if let Some(id_field) = id_field_name {
                quote! {
                    fn id(&self) -> &str {
                        &self.#id_field
                    }
                }
            } else {
                // Fallback: return empty string if no ID field found
                quote! {
                    fn id(&self) -> &str {
                        ""
                    }
                }
            }
        };

        tokens.extend(quote! {
            impl #generic_params_wb opensearch_client::Document for #ident
            #where_clause
            {
                fn index_name() -> &'static str {
                    #index_name
                }

                #id_method

                fn columns() -> Vec<opensearch_client::Field> {
                    vec![#(#document_fields),*]
                }
            }
        });
    }
}

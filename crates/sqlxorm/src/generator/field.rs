use crate::ArgMeta;
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::parse::Parser;
use syn::parse_quote;
#[derive(Debug)]
pub enum FieldDataType {
    Placeholder,
    String,
    I64,
    I32,
    I16,
    Bool,
    Bytes,
    Instant,
    Duration,
    Array(Box<FieldDataType>),
    JsonValue,
}

impl ToTokens for FieldDataType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            FieldDataType::Placeholder => {}
            FieldDataType::String => tokens.extend(quote! {String}),
            FieldDataType::I64 => tokens.extend(quote! {i64}),
            FieldDataType::I32 => tokens.extend(quote! {i32}),
            FieldDataType::I16 => tokens.extend(quote! {i16}),
            FieldDataType::Bool => tokens.extend(quote! {bool}),
            FieldDataType::Bytes => tokens.extend(quote! {Vec<u8>}),
            FieldDataType::Instant => tokens.extend(quote! {::std::time::Instant}),
            FieldDataType::Duration => tokens.extend(quote! {::std::time::Duration}),
            FieldDataType::Array(inner) => tokens.extend(quote! {Vec<#inner>}),
            FieldDataType::JsonValue => tokens.extend(quote! {::serde_json::Value}),
        }
    }
}
#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub column_name: String,
    pub nullable: bool,
    pub data_type: FieldDataType,
}

impl Default for Field {
    fn default() -> Self {
        Self {
            name: Default::default(),
            column_name: Default::default(),
            nullable: true,
            data_type: FieldDataType::Placeholder,
        }
    }
}

impl Field {
    pub fn field_ty(&self) -> TokenStream {
        let field_ty = &self.data_type;
        if self.nullable {
            quote! {Option<#field_ty>}
        } else {
            field_ty.to_token_stream()
        }
    }
    pub fn transform(&mut self, arg: &ArgMeta, field: Option<syn::Field>) -> Result<syn::Field> {
        Ok(
            if field.is_none() || arg.get_bool("generated").unwrap_or_default() {
                let column_name = &self.column_name;
                let field_id = format_ident!("{}", self.name);

                let field_ty = self.field_ty();
                let ts = quote! {
                    #[column(#column_name, generated = true)]
                    #field_id: #field_ty

                };
                let parser = syn::Field::parse_named;
                parser.parse2(ts)?
            } else {
                let field = field.unwrap();
                self.name = field.ident.as_ref().unwrap().to_string();
                field
            },
        )
    }
}

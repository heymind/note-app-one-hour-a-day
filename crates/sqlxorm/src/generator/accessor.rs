use std::collections::HashSet;

use crate::{ArgMeta, Entity};
use anyhow::Result;
use quote::{format_ident, ToTokens};
use syn::{parse::Parse, parse_quote, ImplItemMethod, ItemImpl};

pub fn transform(ent: &mut Entity, item_impl: &mut ItemImpl) -> Result<()> {
    let mut exist_method_names = HashSet::new();

    let mut origin_items = std::mem::take(&mut item_impl.items);
    let mut items = Vec::new();
    for impl_item in origin_items {
        match impl_item {
            syn::ImplItem::Method(item_method) => {
                let ignore = item_method.attrs.iter().any(|attr| {
                    attr.path.is_ident("entity") && {
                        let args: syn::Result<ArgMeta> = attr.parse_args();
                        args.map(|args| {
                            args.is_true("generated")
                                && (args.is_true("setter")
                                    || args.is_true("getter")
                                    || args.is_true("update"))
                        })
                        .unwrap_or_default()
                    }
                });
                if !ignore {
                    exist_method_names.insert(item_method.sig.ident.to_string());
                    items.push(syn::ImplItem::Method(item_method));
                }
            }
            other => items.push(other),
        }
    }
    item_impl.items = items;

    let pkey_field_ident = format_ident!("{}", ent.primary.first().unwrap());

    let mut generated_methods: Vec<ImplItemMethod> = Vec::new();
    for field in &ent.fields {
        let field_ident = format_ident!("{}", &field.name);
        let val_type = field.field_ty();

        let setter_ident = format_ident!("set_{}", &field.name);
        if !exist_method_names.contains(&setter_ident.to_string()) {
            generated_methods.push(parse_quote!{
                #[entity(setter = true, generated = true)]
                pub fn #setter_ident(&mut self,new_value:#val_type) -> ::std::result::Result<(),::std::convert::Infallible> {
                    self.#field_ident = new_value;
                    Ok(())
                }
            });
        }

        let getter_ident = format_ident!("get_{}", &field.name);
        if !exist_method_names.contains(&getter_ident.to_string()) {
            generated_methods.push(parse_quote! {
                #[entity(getter = true, generated = true)]
                pub fn #getter_ident(&self) -> &#val_type {
                    &self.#field_ident
                }
            });
        }

        if !ent.primary.contains(&field.column_name) {
            let update_ident = format_ident!("update_{}", &field.name);
            if !exist_method_names.contains(&setter_ident.to_string()) {
                let sql = format!(
                    "update {} set {} = $1 where {} = $2",
                    &ent.table_name,
                    &field.column_name,
                    ent.primary.first().unwrap()
                );
                generated_methods.push(parse_quote!{
                    #[entity(update = true,generated = true)]
                    pub async fn #update_ident(&mut self,conn: &mut ::sqlx::PgConnection, new_value:#val_type) -> ::std::result::Result<(),Box<dyn ::std::error::Error + Send + Sync>> {
                        self.#setter_ident(new_value)?;
                        ::sqlx::query(#sql).bind(self.#getter_ident()).bind(&self.#pkey_field_ident).execute(conn).await?;
                        Ok(())
                    }
                });
            }
        }
    }
    item_impl
        .items
        .extend(generated_methods.drain(..).map(syn::ImplItem::Method));

    Ok(())
}

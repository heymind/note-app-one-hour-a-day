use std::collections::HashSet;

use crate::{ArgMeta, Entity};
use anyhow::Result;
use quote::{format_ident, quote, ToTokens};
use std::fmt::Write;
use syn::{parse::Parse, parse_quote, ImplItemMethod, ItemImpl};
pub fn transform(ent: &mut Entity, item_impl: &mut ItemImpl) -> Result<()> {
    let mut exist_method_names = HashSet::new();
    let mut savers: Vec<(String, ArgMeta)> = Vec::new();

    let mut origin_items = std::mem::take(&mut item_impl.items);
    let mut items = Vec::new();
    for impl_item in origin_items {
        match impl_item {
            syn::ImplItem::Method(item_method) => {
                let save_args: Option<ArgMeta> = item_method.attrs.iter().find_map(|attr| {
                    if attr.path.is_ident("entity") {
                        let args: syn::Result<ArgMeta> = attr.parse_args();
                        if args
                            .as_ref()
                            .map(|args| args.is_true("generated") && args.has("save"))
                            .unwrap_or_default()
                        {
                            return Some(args.unwrap());
                        }
                    }
                    None
                });
                if let Some(args) = save_args {
                    savers.push((item_method.sig.ident.to_string(), args));
                } else {
                    exist_method_names.insert(item_method.sig.ident.to_string());
                    items.push(syn::ImplItem::Method(item_method));
                }
            }
            other => items.push(other),
        }
    }
    item_impl.items = items;

    if !savers.iter().any(|(n, _)| n == "save") && !exist_method_names.contains("save") {
        savers.push(("save".to_string(), Default::default()));
    }

    let pkey_field_ident = format_ident!("{}", ent.primary.first().unwrap());

    let mut generated_methods: Vec<ImplItemMethod> = Vec::new();

    for (method_name, args) in savers {
        let mut column_names = args.get_strings("save");
        if column_names.is_empty() {
            column_names = ent
                .fields
                .iter()
                .filter_map(|f| {
                    if ent.primary.contains(&f.column_name) {
                        None
                    } else {
                        Some(f.column_name.clone())
                    }
                })
                .collect();
        }
        let mut bindings = Vec::new();
        let mut sql = format!("update {} set", ent.table_name);
        for (id, name) in column_names.iter().enumerate() {
            let field = ent
                .fields
                .iter()
                .find(|f| &f.column_name == name)
                .expect("column not found");

            write!(sql, " {} = ${} ,", name, id + 1)?;
            let getter_ident = format_ident!("get_{}", field.name);
            bindings.push(quote!(.bind(self.#getter_ident())));
        }
        sql.pop();
        write!(
            sql,
            " where {} = ${}",
            ent.primary.first().unwrap(),
            column_names.len() + 1
        )?;

        let method_ident = format_ident!("{}", method_name);
        generated_methods.push(parse_quote!{
            #[entity(save = [#(#column_names),*],generated = true)]
            pub async fn #method_ident(&mut self,conn: &mut ::sqlx::PgConnection) -> ::std::result::Result<(),::sqlx::Error> {
                let sql = #sql;
                ::sqlx::query(sql)#(#bindings)*.bind(&self.#pkey_field_ident).execute(conn).await?;
                Ok(())
            }
        });
    }

    item_impl
        .items
        .extend(generated_methods.drain(..).map(syn::ImplItem::Method));

    Ok(())
}

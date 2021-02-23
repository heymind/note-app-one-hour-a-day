use std::iter::FromIterator;

use crate::field;

use super::{ArgMeta, Field, FieldDataType};
use anyhow::{bail, Result};
use syn::{punctuated::Punctuated, Fields, ItemStruct, Token};
#[derive(Debug, Default)]
pub struct Entity {
    pub table_name: String,
    pub primary: Vec<String>,
    pub fields: Vec<Field>,
    pub item_name: String,
}

impl Entity {
    pub fn transform(&mut self, arg: &ArgMeta, item: &mut ItemStruct) -> Result<()> {
        self.item_name = item.ident.to_string();
        if let Fields::Named(named_fields) = &mut item.fields {
            let mut generated: Vec<bool> = (0..self.fields.len()).map(|_| false).collect();
            let fields = std::mem::take(&mut named_fields.named);
            let mut fields_after: Vec<syn::Field> = Vec::new();
            for field in fields.into_pairs().map(|p| p.into_value()) {
                let mut field_args = field.attrs.iter().find(|attr| attr.path.is_ident("column"));
                let field_args: ArgMeta = if let Some(field_args) = field_args {
                    field_args.parse_args()?
                } else {
                    Default::default()
                };
                let column_name = field_args
                    .get_str(0)
                    .unwrap_or_else(|| (&field.ident).as_ref().unwrap().to_string());

                let idx = self.fields.iter().enumerate().find_map(|(idx, field)| {
                    if field.column_name == column_name {
                        Some(idx)
                    } else {
                        None
                    }
                });

                let field = if let Some(found_idx) = idx {
                    generated[found_idx] = true;
                    self.fields[found_idx].transform(&field_args, Some(field))?
                } else {
                    field
                };
                fields_after.push(field);
            }
            for (idx, generated) in generated.drain(..).enumerate() {
                if !generated {
                    fields_after.push(self.fields[idx].transform(&Default::default(), None)?);
                }
            }
            named_fields.named = Punctuated::<syn::Field, Token![,]>::from_iter(fields_after);
        } else {
            bail!("only support named struct {}", item.ident)
        }

        Ok(())
    }
}

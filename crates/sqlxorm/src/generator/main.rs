use std::io::{Read, Seek};

use accessor::transform;
use anyhow::Result;
use quote::ToTokens;
use sqlx::postgres::PgPoolOptions;
mod arg_meta;
pub use arg_meta::ArgMeta;
mod registry;
use registry::Registry;
mod entity;
use entity::Entity;
mod field;
use field::{Field, FieldDataType};
mod column_def;
use column_def::ColumnDef;
mod accessor;
mod query_builder;
mod save;
use std::io::SeekFrom;
use std::io::Write;
use syn::{
    parse::Parse,
    visit_mut::{self, VisitMut},
    File, ItemImpl, ItemStruct,
};
struct Vis(Registry);

impl visit_mut::VisitMut for Vis {
    fn visit_item_struct_mut(&mut self, i: &mut ItemStruct) {
        let attr = i.attrs.iter().find(|a| a.path.is_ident("entity"));
        if attr.is_none() {
            return;
        }
        let arg: ArgMeta = attr.unwrap().parse_args().unwrap();
        let table_name = arg.get_str("table_name").unwrap();
        if let Some(entity) = self.0.entities.get_mut(&table_name) {
            entity.transform(&arg, i).unwrap();
        } else {
            panic!("table {} not found", table_name);
        }
    }
    fn visit_item_impl_mut(&mut self, i: &mut ItemImpl) {
        let attr = i.attrs.iter().find(|a| a.path.is_ident("entity"));
        if attr.is_none() {
            return;
        }
        let entity = self
            .0
            .entities
            .values_mut()
            .find(|ent| match i.self_ty.as_ref() {
                syn::Type::Path(tp) => tp.path.is_ident(&ent.item_name),
                _ => false,
            })
            .expect("struct not found");
        accessor::transform(entity, i).unwrap();
        save::transform(entity, i).unwrap();
    }
}
#[tokio::main]
async fn main() -> Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/marks")
        .await?;

    let mut conn = pool.acquire().await?;

    let reg = registry::Registry::load(&mut conn, "public").await?;

    let mut f = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open("examples/simple.rs")?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;

    let mut file: File = syn::parse_file(&content)?;

    let mut vis = Vis(reg);
    vis.visit_file_mut(&mut file);

    let new_content = file.to_token_stream().to_string();
    f.set_len(0)?;
    f.seek(SeekFrom::Start(0))?;
    write!(f, "{}", new_content)?;
    //    println!("{}",new_content);

    // dbg!(reg);

    Ok(())
}

use super::{ColumnDef, Entity, Field};
use anyhow::Result;
use sqlx::PgConnection;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Registry {
    pub entities: HashMap<String, Entity>,
}

impl Registry {
    pub async fn load(conn: &mut PgConnection, schema: &str) -> Result<Self> {
        let mut entities: HashMap<String, Entity> = Default::default();
        let columns = ColumnDef::find_by_schema(conn, schema).await?;

        for column in columns {
            let data_type = column.data_type()?;
            let is_primary = column.is_primary();
            let nullable = column.nullable();

            let name = column.column_name;
            let table_name = column.table_name;

            let field = Field {
                column_name: name.clone(),
                name,
                nullable,
                data_type,
            };

            if let Some(entity) = entities.get_mut(&table_name) {
                if is_primary {
                    entity.primary.push(field.name.clone());
                }
                entity.fields.push(field);
            } else {
                let entity = Entity {
                    table_name: table_name.clone(),
                    primary: if is_primary {
                        vec![field.name.clone()]
                    } else {
                        Default::default()
                    },
                    fields: vec![field],
                    ..Default::default()
                };
                entities.insert(table_name, entity);
            }
        }

        Ok(Self { entities })
    }
}

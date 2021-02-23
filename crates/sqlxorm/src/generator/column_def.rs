use super::FieldDataType;
use anyhow::Result;
use sqlx::{FromRow, PgConnection};
#[derive(Debug, FromRow)]
pub struct ColumnDef {
    pub table_name: String,
    pub column_name: String,
    pub is_nullable: String,
    pub udt_name: String,
    pub constraint_type: Option<String>,
}

impl ColumnDef {
    pub async fn find_by_schema(conn: &mut PgConnection, schema: &str) -> Result<Vec<Self>> {
        Ok(sqlx::query_as(
            r#"
            SELECT
                table_name,
                column_name,
                is_nullable,
                udt_name,
                constraint_name,
                constraint_type
            FROM
                information_schema.columns
                NATURAL LEFT JOIN information_schema.constraint_column_usage
                NATURAL LEFT JOIN information_schema.table_constraints  WHERE
                table_schema = $1
           ;"#,
        )
        .bind(schema)
        .fetch_all(conn)
        .await?)
    }

    pub fn nullable(&self) -> bool {
        self.is_nullable == "YES"
    }

    pub fn is_primary(&self) -> bool {
        if let Some(t) = &self.constraint_type {
            t == "PRIMARY KEY"
        } else {
            false
        }
    }

    pub fn data_type(&self) -> Result<FieldDataType> {
        fn parse_dt(dt: &str) -> Result<FieldDataType> {
            Ok(match dt {
                "int8" => FieldDataType::I64,
                "int4" => FieldDataType::I32,
                "int2" => FieldDataType::I16,
                "text" | "varchar" | "bpchar" => FieldDataType::String,
                "bool" => FieldDataType::Bool,
                "jsonb" => FieldDataType::JsonValue,
                "timestamptz" | "date" => FieldDataType::Instant,
                "interval" => FieldDataType::Duration,
                other if other.starts_with('_') => {
                    FieldDataType::Array(Box::new(parse_dt(&other[1..])?))
                }
                others => unimplemented!("unsupport datatype {}", others),
            })
        }
        Ok(parse_dt(&self.udt_name)?)
    }
}

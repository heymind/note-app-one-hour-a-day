use sqlx::PgConnection;
#[entity(table_name = "marble_auth_perms")]
pub struct User {
    #[column("name")]
    name: String,
    #[column("description")]
    description: String,
    #[column("eee", generated = true)]
    eee: Option<::serde_json::Value>,
    #[column("yyyy", generated = true)]
    yyyy: Option<Vec<String>>,
    #[column("zzz", generated = true)]
    zzz: Option<Vec<i32>>,
    #[column("xxx", generated = true)]
    xxx: Option<Vec<String>>,
    #[column("is_deprecated", generated = true)]
    is_deprecated: bool,
    #[column("object_kind_id", generated = true)]
    object_kind_id: i64,
    #[column("id", generated = true)]
    id: i64,
}
impl User {
    #[query]
    #[query]
    async fn find_by_id(conn: &mut PgConnection, id: usize) -> Result<Self>;
    #[query("select * from marble_auth_perms where name = :1")]
    async fn find_by_name(conn: &mut PgConnection, name: &str) -> Result<Self>;
}

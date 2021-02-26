use sqlx::PgConnection;
#[entity(table_name = "marble_auth_perms")]
pub struct User {
    #[column("is_deprecated", generated = true)]
    is_deprecated: bool,
    #[column("name", generated = true)]
    name: String,
    #[column("object_kind_id", generated = true)]
    object_kind_id: i64,
    #[column("id", generated = true)]
    id: i64,
    #[column("description", generated = true)]
    description: String,
    #[column("eee", generated = true)]
    eee: Option<::serde_json::Value>,
    #[column("yyyy", generated = true)]
    yyyy: Option<Vec<String>>,
    #[column("zzz", generated = true)]
    zzz: Option<Vec<i32>>,
    #[column("xxx", generated = true)]
    xxx: Option<Vec<String>>,
}
#[entity]
impl User {
    #[query]
    async fn find_by_id(conn: &mut PgConnection, id: usize) -> Result<Self>;
    #[query("select * from marble_auth_perms where name = :1")]
    async fn find_by_name(conn: &mut PgConnection, name: &str) -> Result<Self>;
    #[entity(generated = true)]
    pub fn set_is_deprecated(
        &mut self,
        new_value: bool,
    ) -> ::std::result::Result<(), ::std::convert::Infallible> {
        self.is_deprecated = new_value;
        Ok(())
    }
    #[entity(generated = true)]
    pub fn get_is_deprecated(&self) -> &bool {
        &self.is_deprecated
    }
    #[entity(generated = true)]
    pub async fn update_is_deprecated(
        &mut self,
        conn: &mut ::sqlx::PgConnection,
        new_value: bool,
    ) -> ::std::result::Result<(), Box<dyn ::std::error::Error + Send + Sync>> {
        self.set_is_deprecated(new_value)?;
        ::sqlx::query("update marble_auth_perms set is_deprecated = $1 where id = $2")
            .bind(self.get_is_deprecated())
            .bind(&self.id)
            .execute(conn)
            .await?;
        Ok(())
    }
    #[entity(generated = true)]
    pub fn set_name(
        &mut self,
        new_value: String,
    ) -> ::std::result::Result<(), ::std::convert::Infallible> {
        self.name = new_value;
        Ok(())
    }
    #[entity(generated = true)]
    pub fn get_name(&self) -> &String {
        &self.name
    }
    #[entity(generated = true)]
    pub async fn update_name(
        &mut self,
        conn: &mut ::sqlx::PgConnection,
        new_value: String,
    ) -> ::std::result::Result<(), Box<dyn ::std::error::Error + Send + Sync>> {
        self.set_name(new_value)?;
        ::sqlx::query("update marble_auth_perms set name = $1 where id = $2")
            .bind(self.get_name())
            .bind(&self.id)
            .execute(conn)
            .await?;
        Ok(())
    }
    #[entity(generated = true)]
    pub fn set_object_kind_id(
        &mut self,
        new_value: i64,
    ) -> ::std::result::Result<(), ::std::convert::Infallible> {
        self.object_kind_id = new_value;
        Ok(())
    }
    #[entity(generated = true)]
    pub fn get_object_kind_id(&self) -> &i64 {
        &self.object_kind_id
    }
    #[entity(generated = true)]
    pub async fn update_object_kind_id(
        &mut self,
        conn: &mut ::sqlx::PgConnection,
        new_value: i64,
    ) -> ::std::result::Result<(), Box<dyn ::std::error::Error + Send + Sync>> {
        self.set_object_kind_id(new_value)?;
        ::sqlx::query("update marble_auth_perms set object_kind_id = $1 where id = $2")
            .bind(self.get_object_kind_id())
            .bind(&self.id)
            .execute(conn)
            .await?;
        Ok(())
    }
    #[entity(generated = true)]
    pub fn set_id(
        &mut self,
        new_value: i64,
    ) -> ::std::result::Result<(), ::std::convert::Infallible> {
        self.id = new_value;
        Ok(())
    }
    #[entity(generated = true)]
    pub fn get_id(&self) -> &i64 {
        &self.id
    }
    #[entity(generated = true)]
    pub fn set_description(
        &mut self,
        new_value: String,
    ) -> ::std::result::Result<(), ::std::convert::Infallible> {
        self.description = new_value;
        Ok(())
    }
    #[entity(generated = true)]
    pub fn get_description(&self) -> &String {
        &self.description
    }
    #[entity(generated = true)]
    pub async fn update_description(
        &mut self,
        conn: &mut ::sqlx::PgConnection,
        new_value: String,
    ) -> ::std::result::Result<(), Box<dyn ::std::error::Error + Send + Sync>> {
        self.set_description(new_value)?;
        ::sqlx::query("update marble_auth_perms set description = $1 where id = $2")
            .bind(self.get_description())
            .bind(&self.id)
            .execute(conn)
            .await?;
        Ok(())
    }
    #[entity(generated = true)]
    pub fn set_eee(
        &mut self,
        new_value: Option<::serde_json::Value>,
    ) -> ::std::result::Result<(), ::std::convert::Infallible> {
        self.eee = new_value;
        Ok(())
    }
    #[entity(generated = true)]
    pub fn get_eee(&self) -> &Option<::serde_json::Value> {
        &self.eee
    }
    #[entity(generated = true)]
    pub async fn update_eee(
        &mut self,
        conn: &mut ::sqlx::PgConnection,
        new_value: Option<::serde_json::Value>,
    ) -> ::std::result::Result<(), Box<dyn ::std::error::Error + Send + Sync>> {
        self.set_eee(new_value)?;
        ::sqlx::query("update marble_auth_perms set eee = $1 where id = $2")
            .bind(self.get_eee())
            .bind(&self.id)
            .execute(conn)
            .await?;
        Ok(())
    }
    #[entity(generated = true)]
    pub fn set_yyyy(
        &mut self,
        new_value: Option<Vec<String>>,
    ) -> ::std::result::Result<(), ::std::convert::Infallible> {
        self.yyyy = new_value;
        Ok(())
    }
    #[entity(generated = true)]
    pub fn get_yyyy(&self) -> &Option<Vec<String>> {
        &self.yyyy
    }
    #[entity(generated = true)]
    pub async fn update_yyyy(
        &mut self,
        conn: &mut ::sqlx::PgConnection,
        new_value: Option<Vec<String>>,
    ) -> ::std::result::Result<(), Box<dyn ::std::error::Error + Send + Sync>> {
        self.set_yyyy(new_value)?;
        ::sqlx::query("update marble_auth_perms set yyyy = $1 where id = $2")
            .bind(self.get_yyyy())
            .bind(&self.id)
            .execute(conn)
            .await?;
        Ok(())
    }
    #[entity(generated = true)]
    pub fn set_zzz(
        &mut self,
        new_value: Option<Vec<i32>>,
    ) -> ::std::result::Result<(), ::std::convert::Infallible> {
        self.zzz = new_value;
        Ok(())
    }
    #[entity(generated = true)]
    pub fn get_zzz(&self) -> &Option<Vec<i32>> {
        &self.zzz
    }
    #[entity(generated = true)]
    pub async fn update_zzz(
        &mut self,
        conn: &mut ::sqlx::PgConnection,
        new_value: Option<Vec<i32>>,
    ) -> ::std::result::Result<(), Box<dyn ::std::error::Error + Send + Sync>> {
        self.set_zzz(new_value)?;
        ::sqlx::query("update marble_auth_perms set zzz = $1 where id = $2")
            .bind(self.get_zzz())
            .bind(&self.id)
            .execute(conn)
            .await?;
        Ok(())
    }
    #[entity(generated = true)]
    pub fn set_xxx(
        &mut self,
        new_value: Option<Vec<String>>,
    ) -> ::std::result::Result<(), ::std::convert::Infallible> {
        self.xxx = new_value;
        Ok(())
    }
    #[entity(generated = true)]
    pub fn get_xxx(&self) -> &Option<Vec<String>> {
        &self.xxx
    }
    #[entity(generated = true)]
    pub async fn update_xxx(
        &mut self,
        conn: &mut ::sqlx::PgConnection,
        new_value: Option<Vec<String>>,
    ) -> ::std::result::Result<(), Box<dyn ::std::error::Error + Send + Sync>> {
        self.set_xxx(new_value)?;
        ::sqlx::query("update marble_auth_perms set xxx = $1 where id = $2")
            .bind(self.get_xxx())
            .bind(&self.id)
            .execute(conn)
            .await?;
        Ok(())
    }
    # [entity (save = ["id" , "name"] , generated = true)]
    pub async fn save_something(
        &mut self,
        conn: &mut ::sqlx::PgConnection,
    ) -> ::std::result::Result<(), ::sqlx::Error> {
        let sql = "update marble_auth_perms set id = $1 , name = $2  where id = $3";
        ::sqlx::query(sql)
            .bind(self.get_id())
            .bind(self.get_name())
            .bind(&self.id)
            .execute(conn)
            .await?;
        Ok(())
    }
    # [entity (save = ["is_deprecated" , "name" , "object_kind_id" , "id" , "description" , "eee" , "yyyy" , "zzz" , "xxx"] , generated = true)]
    pub async fn save(
        &mut self,
        conn: &mut ::sqlx::PgConnection,
    ) -> ::std::result::Result<(), ::sqlx::Error> {
        let sql = "update marble_auth_perms set is_deprecated = $1 , name = $2 , object_kind_id = $3 , id = $4 , description = $5 , eee = $6 , yyyy = $7 , zzz = $8 , xxx = $9  where id = $10" ;
        ::sqlx::query(sql)
            .bind(self.get_is_deprecated())
            .bind(self.get_name())
            .bind(self.get_object_kind_id())
            .bind(self.get_id())
            .bind(self.get_description())
            .bind(self.get_eee())
            .bind(self.get_yyyy())
            .bind(self.get_zzz())
            .bind(self.get_xxx())
            .bind(&self.id)
            .execute(conn)
            .await?;
        Ok(())
    }
}

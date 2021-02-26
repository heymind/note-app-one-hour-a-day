pub struct QueryName {
    pub select: Option<String>,
    pub r#where: Vec<(String, String, String)>,
    pub by: Vec<String>,
    pub order_by: Vec<(String, bool)>,
    pub limit: Option<Option<usize>>,
    pub offset: Option<Option<usize>>,
}
